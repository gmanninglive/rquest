use crate::{http::*, AppState};
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use entity::prelude::*;
use serde::Serialize;
use uuid::Uuid;

async fn question(
    State(state): State<AppState>,
    Path(thread_id): Path<Uuid>,
) -> Result<Json<Message>> {
    Ok(Json(Message::find_as_question(&state.db, thread_id).await?))
}

async fn one(
    State(state): State<AppState>,
    Path(thread_id): Path<Uuid>,
) -> Result<Json<Thread>> {
    Ok(Json(Thread::find_by_id(&state.db, thread_id).await?))
}

#[derive(Clone, Serialize, sqlx::FromRow)]
struct ThreadWithRelated {
    thread: Option<Thread>,
    event: Option<Event>,
    question: Option<Message>,
    //answer: Option<message::Model>,
}

async fn related(
    State(state): State<AppState>,
    Path(thread_id): Path<Uuid>,
) -> Result<Json<ThreadWithRelated>> {
    let query = sqlx::query_as!(
        ThreadWithRelated,
        r#"
            select 
                thread as "thread: Thread",
                question as "question: Message",
                event as "event: Event"
            from thread
                inner join question q on q.id = thread.question_id
                left join message question on question.id = q.id
                left join event on event.id = thread.event_id
            where thread.id = $1
            "#,
        thread_id
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(query))
}

async fn index(State(state): State<AppState>) -> Result<Json<Vec<Thread>>> {
    Ok(Json(Thread::find_all(&state.db).await?))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/threads", get(index))
        .route("/thread/:id", get(one))
        .route("/thread/:id/question", get(question))
        .route("/thread/:id/related", get(related))
}
