use crate::http::extractor::AuthUser;
use crate::http::{Error, Result};
use crate::AppState;
use anyhow::Context;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash};
use axum::{
    extract::{Path, State},
    routing::{delete, get, patch, post},
    Json, Router,
};
use entity::user;
use entity::user::Entity as User;
use sea_orm::entity::*;
use sea_orm::{EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub async fn hash_password(password: String) -> Result<String> {
    // Argon2 hashing is designed to be computationally intensive,
    // so we need to do this on a blocking thread.
    Ok(tokio::task::spawn_blocking(move || -> Result<String> {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(
            PasswordHash::generate(Argon2::default(), password, salt.as_str())
                .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
                .to_string(),
        )
    })
    .await
    .context("panic in generating password hash")??)
}

async fn verify_password(password: String, password_hash: String) -> Result<()> {
    Ok(tokio::task::spawn_blocking(move || -> Result<()> {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;

        hash.verify_password(&[&Argon2::default()], password)
            .map_err(|e| match e {
                argon2::password_hash::Error::Password => Error::Unauthorized,
                _ => anyhow::anyhow!("failed to verify password hash: {}", e).into(),
            })
    })
    .await
    .context("panic in verifying password hash")??)
}

#[derive(Deserialize, Serialize)]
struct UserSession {
    id: Uuid,
    email: String,
    username: String,
    image: Option<String>,
    token: String,
}

#[derive(Deserialize)]
struct LoginUser {
    email: String,
    password: String,
}

// https://realworld-docs.netlify.app/docs/specs/backend-specs/endpoints#authentication
async fn login_user(
    State(state): State<AppState>,
    Json(req): Json<LoginUser>,
) -> Result<Json<UserSession>> {
    let user = User::find()
        .filter(user::Column::Email.eq(req.email))
        .one(&state.db)
        .await?;

    match user {
        Some(user) => {
            verify_password(req.password, user.password_hash).await?;

            Ok(Json(UserSession {
                id: user.id,
                email: user.email,
                token: AuthUser { user_id: user.id }.to_jwt(&state),
                username: user.username,
                image: user.image,
            }))
        }
        None => Err(Error::NotFound("user")),
    }
}

async fn get_current_user(
    auth_user: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<UserSession>> {
    let user = User::find()
        .filter(user::Column::Id.eq(auth_user.user_id))
        .one(&state.db)
        .await?;

    match user {
        Some(user) => Ok(Json(UserSession {
            id: user.id,
            email: user.email,
            token: AuthUser { user_id: user.id }.to_jwt(&state),
            username: user.username,
            image: user.image,
        })),

        None => Err(Error::NotFound("user")),
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/session/new", post(login_user))
        .route("/session", get(get_current_user))
}
