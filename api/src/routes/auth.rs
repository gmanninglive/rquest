use crate::http::extractor::AuthUser;
use crate::http::Result;
use crate::AppState;
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use core::auth::verify_password;
use core::query::user::{user, UserQuery};
use sea_orm::entity::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

async fn login_user(
    State(state): State<AppState>,
    Json(req): Json<LoginUser>,
) -> Result<Json<UserSession>> {
    let user = UserQuery::find_by(&state.db, user::Column::Email.eq(req.email)).await?;

    verify_password(req.password, user.password_hash).await?;

    Ok(Json(UserSession {
        id: user.id,
        email: user.email,
        token: AuthUser { id: user.id }.to_jwt(&state),
        username: user.username,
        image: user.image,
    }))
}

async fn get_current_user(
    auth_user: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<UserSession>> {
    let user = UserQuery::find(&state.db, auth_user.id).await?;

    Ok(Json(UserSession {
        id: user.id,
        email: user.email,
        token: AuthUser { id: user.id }.to_jwt(&state),
        username: user.username,
        image: user.image,
    }))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/session/new", post(login_user))
        .route("/session", get(get_current_user))
}
