use rquest_core::http::*;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::message;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    sqlx::FromRow,
    sqlx::Type,
)]
pub struct Model {
    pub id: Uuid,
    pub question_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub event_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct UpdateParams {
    answer_id: Uuid,
}

#[derive(Deserialize)]
pub struct CreateQuestionParams {
    event_id: Uuid,
    question: message::CreateParams,
}

impl Model {
    pub async fn find_by_id(db: &PgPool, thread_id: Uuid) -> Result<Model> {
        Ok(sqlx::query_as!(
            Model,
            r#"select * from thread where thread.id = $1"#,
            thread_id
        )
        .fetch_one(db)
        .await?)
    }
    //pub async fn find_as_answer(db: &PgPool, as_answer_id: Uuid) -> Result<Model> {
        //Ok(sqlx::query_as!(
            //Model,
            //r#"select * from thread where thread.answer_id = $1"#,
            //as_answer_id
        //)
        //.fetch_one(db)
        //.await?)
    //}
    pub async fn find_all(db: &PgPool) -> Result<Vec<Model>> {
        let query = sqlx::query_as!(Model, r#"select * from thread"#);

        Ok(query.fetch_all(db).await?)
    }
    pub async fn create_question(db: &PgPool, req: CreateQuestionParams) -> Result<Model> {
        let question = sqlx::query_as!(
            Model,
            r#"
            with created_message as (
                insert into message (text, state, user_id)
                values ($1, $2, $3)
                returning id
            ), 
            created_thread as (
                insert into thread (event_id)
                values ($4)
                returning id
            ),
            created_question as (
                insert into question (thread_id, message_id)
                values (
                    (select id from created_thread), 
                    (select id from created_message)
                )
                returning id 
            )
            update thread
                set question_id = (select id from created_question)
            from created_thread
            where thread.id = created_thread.id 
            returning thread.*
            "#,
            req.question.text,
            Into::<i16>::into(req.question.publish.unwrap_or(false)),
            req.question.user_id,
            req.event_id
        )
        .fetch_one(db)
        .await?;
        Ok(question)
    }
    //pub async fn create(db: &DbConn, question_id: Uuid) -> Result<Model> {
    //}
    //pub async fn update(db: &PgPool, thread_id: Uuid, req: UpdateParams) -> Result<Model> {
        //let thread = sqlx::query_as!(
            //Model,
            //r#"update thread
                            //set answer_id = $2
                        //where thread.id = $1
                        //returning *
                    //"#,
            //thread_id,
            //req.answer_id
        //);
        //Ok(thread.fetch_one(db).await?)
    //}
}
