use crate::{http::*, AppState};
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use entity::{
    message, message::Entity as Message, session, thread, thread::Entity as Thread
};
use serde::Serialize;
use uuid::Uuid;

async fn question(
    State(state): State<AppState>,
    Path(thread_id): Path<Uuid>,
) -> Result<Json<message::Model>> {
    Ok(Json(
        Message::find_as_question(&state.db, thread_id).await?
    ))
}

async fn one(
    State(state): State<AppState>,
    Path(thread_id): Path<Uuid>,
) -> Result<Json<thread::Model>> {
    Ok(Json(Thread::find_by_id(&state.db, thread_id).await?))
}

#[derive(Clone, Serialize, sqlx::FromRow)]
struct ThreadWithRelated {
    thread: Option<thread::Model>,
    session: Option<session::Model>,
    question: Option<message::Model>,
    answer: Option<message::Model>,
}

async fn related(
    State(state): State<AppState>,
    Path(thread_id): Path<Uuid>,
) -> Result<Json<ThreadWithRelated>> {
    let query = sqlx::query_as!(
        ThreadWithRelated,
        r#"
            select 
            thread as "thread: thread::Model",
            question as "question: message::Model",
            answer as "answer: message::Model",
            session as "session: session::Model"
            from thread
            left join message question on question.id = thread.question_id
            left join message answer on answer.id = thread.answer_id
            left join "session" on session.id = thread.session_id
            where thread.id = $1
            "#,
        thread_id
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(query))
}

async fn index(State(state): State<AppState>) -> Result<Json<Vec<thread::Model>>> {
    Ok(Json(Thread::find_all(&state.db).await?))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/threads", get(index))
        .route("/thread/:id", get(one))
        .route("/thread/:id/question", get(question))
        .route("/thread/:id/related", get(related))
}
