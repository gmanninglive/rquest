use crate::{http::*, AppState};
use axum::{
    extract::{Path, State},
    routing::{delete, get, patch, post},
    Json, Router,
};
use entity::{user, user::Entity as User};
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait, Set};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
struct UserBody {
    username: String,
    email: String,
}
#[derive(Deserialize)]
struct UpdateBody {
    username: Option<String>,
    email: Option<String>,
    image: Option<String>,
}

async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<UserBody>,
) -> Result<Json<user::Model>> {
    let user = user::ActiveModel {
        username: ActiveValue::Set(req.username),
        email: ActiveValue::Set(req.email),
        ..Default::default()
    }
    .insert(&state.db).await?;

    Ok(Json(user))
}

async fn find_by_id(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<user::Model>> {
    let user = User::find_by_id(user_id).one(&state.db).await?;

    match user {
        Some(u) => Ok(Json(u)),
        None => Err(Error::NotFound("user")),
    }
}

async fn update_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<UpdateBody>,
) -> Result<Json<user::Model>> {
    let user = User::find_by_id(user_id)
        .one(&state.db)
        .await?
        .ok_or(Error::NotFound("user"))?;

    let res = user::ActiveModel {
        id: ActiveValue::Unchanged(user.id),
        username: match req.username {
            Some(username) => Set(username),
            None => ActiveValue::Unchanged(user.username),
        },
        email: match req.email {
            Some(email) => Set(email),
            None => ActiveValue::Unchanged(user.email),
        },
        image: match req.image {
            Some(image) => Set(Some(image)),
            None => ActiveValue::Unchanged(user.image),
        },
        ..Default::default()
    }
    .update(&state.db)
    .await?;

    Ok(Json(res))
}

async fn delete_user(State(state): State<AppState>, Path(user_id): Path<Uuid>) -> Result<()> {
    let user: user::ActiveModel = User::find_by_id(user_id)
        .one(&state.db)
        .await?
        .ok_or(Error::NotFound("user"))
        .map(Into::into)?;

    user.delete(&state.db).await?;
    Ok(())
}

async fn index(State(state): State<AppState>) -> Result<Json<Vec<user::Model>>> {
    let users = User::find().all(&state.db).await?;

    Ok(Json(users))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users", get(index))
        .route("/user/:user_id", get(find_by_id))
        .route("/user/update/:user_id", patch(update_user).put(update_user))
        .route("/user/delete/:user_id", delete(delete_user))
        .route("/user/new", post(create_user))
}
