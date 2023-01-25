use crate::{http::*, AppState};
use axum::{
    extract::{Path, State},
    routing::{delete, get, patch, post},
    Json, Router,
};
use core::mutation::user::{CreateParams, UpdateParams, UserMutation};
use core::query::user::UserQuery;
use entity::user;
use uuid::Uuid;

async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateParams>,
) -> Result<Json<user::Model>> {
    Ok(Json(UserMutation::create(&state.db, req).await?))
}

async fn find_by_id(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<user::Model>> {
    Ok(Json(UserQuery::find(&state.db, user_id).await?))
}

async fn update_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<UpdateParams>,
) -> Result<Json<user::Model>> {
    Ok(Json(UserMutation::update(&state.db, user_id, req).await?))
}

async fn delete_user(State(state): State<AppState>, Path(user_id): Path<Uuid>) -> Result<()> {
    Ok(UserMutation::delete(&state.db, user_id).await?)
}

async fn index(State(state): State<AppState>) -> Result<Json<Vec<user::Model>>> {
    Ok(Json(UserQuery::find_all(&state.db).await?))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users", get(index))
        .route("/user/:user_id", get(find_by_id))
        .route("/user/update/:user_id", patch(update_user).put(update_user))
        .route("/user/delete/:user_id", delete(delete_user))
        .route("/user/new", post(create_user))
}
