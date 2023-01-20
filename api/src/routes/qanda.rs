use serde::{Deserialize,Serialize};
use uuid::Uuid;
use crate::{types::Timestamptz, AppState, http::Result};
use super::user::User;
use axum::{Router, routing::{get,post}};

#[derive(Serialize, Deserialize)]
struct Message {
    user_id: Uuid,
    posted_at: Timestamptz,
    editted_at: Timestamptz,
    text: String,
}

#[derive(Serialize, Deserialize)]
struct Thread {
    question: Message,
    answer: Message,
    comments: Vec<Message>,
}

#[derive(Serialize, Deserialize)]
struct QandA {
    id: Uuid,
    created_at: Timestamptz,
    expires_at: Timestamptz,
    host_id: User,
    users: Vec<User>,
    questions: Vec<Thread>,
}

async fn handler() -> Result<()> {
    Ok(())
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(handler))
}
