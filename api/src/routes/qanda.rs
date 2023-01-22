use crate::{http::Result, AppState};
use axum::{
    routing::{get, post},
    Router,
};

//#[derive(Serialize, Deserialize)]
//struct Message {
//user_id: Uuid,
//posted_at: Timestamptz,
//editted_at: Timestamptz,
//text: String,
//}

//#[derive(Serialize, Deserialize)]
//struct Thread {
//question: Message,
//answer: Message,
//comments: Vec<Message>,
//}

//#[derive(Serialize, Deserialize)]
//struct QandA {
//id: Uuid,
//created_at: Timestamptz,
//expires_at: Timestamptz,
//host_id: User,
//users: Vec<User>,
//questions: Vec<Thread>,
//}

async fn handler() -> Result<()> {
    Ok(())
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(handler))
}
