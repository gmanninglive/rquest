use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
enum UserRole {
    Requester,
    Host,
}

#[derive(Serialize, Deserialize)]
struct User {
    id: Uuid,
    role: UserRole,
    name: String,
    email: String,
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
        id: Uuid::new_v4(),
        role: UserRole::Requester,
        name: "Dave".to_string(),
        email: "example@example.com".to_string(),
    };

    let host = User {
        id: Uuid::new_v4(),
        role: UserRole::Host,
        name: "Theodore".to_string(),
        email: "host@example.com".to_string(),
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
