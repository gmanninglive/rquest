use crate::http::extractor::AuthUser;
use crate::{http::Result, AppState};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{
    extract::{Path, State},
    routing::{delete, get, patch, post},
    Json, Router,
};
use entity::user::{self, CreateParams, Entity as User, Model, UpdateParams};
use uuid::Uuid;

async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateParams>,
) -> Result<impl IntoResponse> {
    Ok((StatusCode::CREATED, Json(User::create(&state.db, req).await?)))
}

async fn find_by_id(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Model>> {
    Ok(Json(User::find_by_id(&state.db, user_id).await?))
}

async fn update_user(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(req): Json<UpdateParams>,
) -> Result<Json<Model>> {
    if req == user::UpdateParams::default() {
        return find_by_id(State(state), Path(auth_user.id)).await;
    }
    Ok(Json(User::update(&state.db, auth_user.id, req).await?))
}

async fn delete_user(State(state): State<AppState>, Path(user_id): Path<Uuid>) -> Result<()> {
    Ok(User::delete(&state.db, user_id).await?)
}

async fn index(State(state): State<AppState>) -> Result<Json<Vec<Model>>> {
    Ok(Json(User::find_all(&state.db).await?))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users", get(index))
        .route("/user/:user_id", get(find_by_id))
        .route("/user/update", patch(update_user).put(update_user))
        .route("/user/delete/:user_id", delete(delete_user))
        .route("/user/new", post(create_user))
}
