use crate::{http::*, AppState};
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
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

//async fn handler() -> impl IntoResponse {
//let user_a = User {
//username: "Dave".to_string(),
//email: "example@example.com".to_string(),
//image: None,
//};

//let host = User {
//username: "Theodore".to_string(),
//email: "host@example.com".to_string(),
//image: None,
//};

//let new_session = QandA {
//id: Uuid::new_v4(),
//created_at: Utc::now(),
//expires_at: Utc::now() + Duration::days(60),
//host,
//users: vec![user_a],
//questions: Vec::new(),
//};

//(StatusCode::OK, Json(new_session))
//}

async fn create_user(State(state): State<AppState>, Json(req): Json<User>) -> Result<Json<User>> {
    sqlx::query_scalar!(
        r#"insert into "user" (username, email) values ($1, $2) returning user_id"#,
        req.username,
        req.email,
    )
    .fetch_one(&state.db)
    .await
    .on_constraint("user_username_key", |_| {
        Error::unprocessable_entity([("username", "username taken")])
    })
    .on_constraint("user_email_key", |_| {
        Error::unprocessable_entity([("email", "email taken")])
    })?;

    Ok(Json(User {
        email: req.email,
        username: req.username,
        image: None,
    }))
}

async fn index(State(state): State<AppState>) -> Result<Json<Vec<User>>> {
    let users = sqlx::query_as::<_, User>(r#"SELECT * FROM "user""#)
        .fetch_all(&state.db)
        .await?;

    Ok(Json(users))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users", get(index))
        .route("/user/create", post(create_user))
}
