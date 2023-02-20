use rquest_core::http::*;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

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
    pub user_id: Option<Uuid>,
    pub text: Option<String>,
    pub state: i16,
    pub created_at: DateTime<Utc>,
    pub posted_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
}

#[derive(
    Debug, Clone, Eq, PartialEq, EnumIter, Serialize, Deserialize, sqlx::Type
)]
#[repr(i16)]
pub enum MessageState {
    Deleted = -1,
    Draft = 0,
    Posted = 1,
}

#[derive(Deserialize)]
pub struct CreateParams {
    pub user_id: Uuid,
    pub text: String,
    pub publish: Option<bool>,
}

impl Model {
    pub async fn find_by_id(db: &PgPool, message_id: Uuid) -> Result<Model> {
        let message = sqlx::query_as!(
            Model,
            r#"
                select * from message
                where message.id = $1
            "#,
            message_id
        )
        .fetch_one(db)
        .await?;

        Ok(message)
    }
    pub async fn find_as_question(db: &PgPool, thread_id: Uuid) -> Result<Model> {
        let question = sqlx::query_as!(
            Model,
            r#"select q.* as "question: Model"
                        from thread
                        left join message q on thread.question_id = q.id
                        where thread.id = $1"#,
            thread_id
        )
        .fetch_one(db)
        .await?;
        Ok(question)
    }
    pub async fn create(db: &PgPool, req: CreateParams) -> Result<Model> {
        let message = sqlx::query_as!(
            Model,
            r#" insert into 
                message (user_id, text, state)
                values($1, $2, $3) 
                returning *
            "#,
            req.user_id,
            req.text,
            Into::<i16>::into(req.publish.unwrap_or(false)),
        );

        Ok(message.fetch_one(db).await?)
    }
}
