use crate::http::extractor::AuthUser;
use crate::{http::*, AppState};
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use entity::message::Entity as Message;
use entity::message::*;
use uuid::Uuid;

async fn new_message(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(req): Json<CreateParams>,
) -> Result<Json<Model>> {
    Ok(Json(
        Message::create(
            &state.db,
            CreateParams {
                user_id: auth_user.id,
                ..req
            },
        )
        .await?,
    ))
}

async fn get_message(
    State(state): State<AppState>,
    Path(message_id): Path<Uuid>,
) -> Result<Json<Model>> {
    Ok(Json(
        <Message as Query<Model>>::find_by_id(&state.db, message_id).await?,
    ))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/message/new", post(new_message))
        .route("/message/:id", get(get_message))
}
