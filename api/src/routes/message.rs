use crate::http::extractor::AuthUser;
use crate::{http::*, AppState};
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use core::{
    mutation::message::{Mutation, NewMessageParams},
    query::message::Query,
};
use entity::message;
use uuid::Uuid;

async fn new_message(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(req): Json<NewMessageParams>,
) -> Result<Json<message::Model>> {
    Ok(Json(Mutation::new(&state.db, req).await?))
}

async fn get_message(
    State(state): State<AppState>,
    Path(message_id): Path<Uuid>,
) -> Result<Json<message::Model>> {
    Ok(Json(Query::find(&state.db, message_id).await?))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/message/new", post(new_message))
        .route("/message/:id", get(get_message))
}
