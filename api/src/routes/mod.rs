use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    email: String,
    image: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Message {
    user_id: Uuid,
    posted_at: DateTime<Utc>,
    editted_at: DateTime<Utc>,
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
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
    host: User,
    users: Vec<User>,
    questions: Vec<Thread>,
}

async fn handler() -> impl IntoResponse {
    let user_a = User {
        username: "Dave".to_string(),
        email: "example@example.com".to_string(),
        image: None,
    };

    let host = User {
        username: "Theodore".to_string(),
        email: "host@example.com".to_string(),
        image: None,
    };

    let new_session = QandA {
        id: Uuid::new_v4(),
        created_at: Utc::now(),
        expires_at: Utc::now() + Duration::days(60),
        host: host,
        users: vec![user_a],
        questions: Vec::new(),
    };

    (StatusCode::OK, Json(new_session))
}

async fn new() -> impl IntoResponse {
    (StatusCode::OK, "test")
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/new", get(new))
}
