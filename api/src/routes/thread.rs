use crate::{http::*, AppState};
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use entity::{message, session, thread, thread::CustomSelectors, thread::Entity as Thread};
use rquest_core::http::Helpers;
use sea_orm::sea_query::{Alias, Expr};
use sea_orm::{entity::prelude::*, ConnectionTrait};
use sea_orm::{EntityTrait, QueryFilter, RelationTrait};
use sea_orm::{QuerySelect, SelectTwo, Set};
use serde::Serialize;
use sqlx::types::{
    chrono::{DateTime, Utc},
    uuid::Uuid,
};
use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    PgPool,
};
use std::collections::HashMap;

async fn question(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Option<message::Model>>> {
    Ok(Json(message::Entity::question(id).one(&state.db).await?))
}

async fn one(
    State(state): State<AppState>,
    Path(thread_id): Path<Uuid>,
) -> Result<Json<thread::Model>> {
    Ok(Json(
        Thread::find_by_id(thread_id)
            .one_or_nf(&state.db, "thread")
            .await?,
    ))
}

//async fn detail(
//State(state): State<AppState>,
//Path(thread_id): Path<Uuid>,
//) -> Result<Json<ThreadWithRelated>> {
//let (thread, question) = Thread::find_by_id(thread_id)
//.with_question()
//.one(&state.db)
//.await?
//.ok_or_else(|| Error::NotFound("thread"))?;

//Ok(Json(ThreadWithRelated {
//id: thread.id,
//question,
//}))
//}

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
    let db_connection_str = "postgres://postgres:postgres@127.0.0.1:5432/rq_dev".to_string();

    // setup connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

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
    .fetch_one(&pool)
    .await?;

    Ok(Json(query))
}

async fn index(State(state): State<AppState>) -> Result<Json<Vec<thread::Model>>> {
    Ok(Json(Thread::find().all(&state.db).await?))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/threads", get(index))
        .route("/thread/:id", get(one))
        .route("/thread/:id/question", get(question))
        .route("/thread/:id/related", get(related))
}
