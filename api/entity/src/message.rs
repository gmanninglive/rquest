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
    DeriveEntityModel,
    Eq,
    Serialize,
    Deserialize,
    sqlx::FromRow,
    sqlx::Type,
)]
#[sea_orm(table_name = "message")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    #[sea_orm(column_type = "Text", nullable)]
    pub text: Option<String>,
    pub state: i16,
    pub created_at: DateTime<Utc>,
    pub posted_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
    pub thread_question_id: Option<Uuid>,
    pub thread_answer_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::thread::Entity",
        from = "Column::ThreadAnswerId",
        to = "super::thread::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    Answer,
    #[sea_orm(
        belongs_to = "super::thread::Entity",
        from = "Column::ThreadQuestionId",
        to = "super::thread::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    Question,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::thread::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Question.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(
    Debug, Clone, Eq, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, sqlx::Type,
)]
#[sea_orm(rs_type = "i16", db_type = "Integer")]
pub enum MessageState {
    Deleted = -1,
    Draft = 0,
    Posted = 1,
}

#[derive(Deserialize)]
pub struct CreateParams {
    pub user_id: Uuid,
    pub text: String,
    pub as_question_thread_id: Option<Uuid>,
    pub as_answer_thread_id: Option<Uuid>,
    pub publish: Option<bool>,
}

impl Entity {
    pub async fn find_by_id(db: &PgPool, message_id: Uuid) -> Result<Model> {
        let message = sqlx::query_as!(
            Model,
            r#"
                            select * from message
                            where message.id = $1"#,
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
                message (user_id, text, state, thread_answer_id, thread_question_id)
                values($1, $2, $3, $4, $5) 
                returning *
            "#,
            req.user_id,
            req.text,
            Into::<i16>::into(req.publish.unwrap_or(false)),
            req.as_answer_thread_id,
            req.as_question_thread_id
        );

        Ok(message.fetch_one(db).await?)
    }
}


