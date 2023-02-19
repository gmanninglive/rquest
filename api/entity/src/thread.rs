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
    DeriveEntityModel,
    Eq,
    Serialize,
    Deserialize,
    sqlx::FromRow,
    sqlx::Type,
)]
#[sea_orm(table_name = "thread")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub question_id: Option<Uuid>,
    pub answer_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub session_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        has_one = "super::message::Entity",
        from = "Column::AnswerId",
        to = "super::message::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    Answer,
    #[sea_orm(
        has_one = "super::message::Entity",
        from = "Column::QuestionId",
        to = "super::message::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    Question,
    #[sea_orm(
        belongs_to = "super::session::Entity",
        from = "Column::SessionId",
        to = "super::session::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    Session,
}

impl Related<super::session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Session.def()
    }
}

impl Related<super::message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Question.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Deserialize)]
pub struct UpdateParams {
    answer_id: Uuid,
}

#[derive(Deserialize)]
pub struct CreateQuestionParams {
    session_id: Uuid,
    question: message::CreateParams,
}

impl Entity {
    pub async fn find_by_id(db: &PgPool, thread_id: Uuid) -> Result<Model> {
        Ok(sqlx::query_as!(
            Model,
            r#"select * from thread where thread.id = $1"#,
            thread_id
        )
        .fetch_one(db)
        .await?)
    }
    pub async fn find_as_answer(db: &PgPool, as_answer_id: Uuid) -> Result<Model> {
        Ok(sqlx::query_as!(
            Model,
            r#"select * from thread where thread.answer_id = $1"#,
            as_answer_id
        )
        .fetch_one(db)
        .await?)
    }
    pub async fn find_all(db: &PgPool) -> Result<Vec<Model>> {
        let query = sqlx::query_as!(Model, r#"select * from thread"#);

        Ok(query.fetch_all(db).await?)
    }
    pub async fn create_question(db: &PgPool, req: CreateQuestionParams) -> Result<Model> {
        let question = sqlx::query_as!(
            Model,
            r#"
                    with created_question as (
                        insert into message (text, state, user_id)
                        values ($1, $2, $3)
                        returning id
                    ),
                    created_thread as (
                        insert into thread (session_id, question_id)
                        values ($4, (select id from created_question))
                        returning *
                    ), updated_question as (
                        update message 
                        set thread_question_id = (select id from created_thread)
                        where message.id = (select id from created_question)
                    )
                    select * from created_thread
                    "#,
            req.question.text,
            Into::<i16>::into(req.question.publish.unwrap_or(false)),
            req.question.user_id,
            req.session_id
        )
        .fetch_one(db)
        .await?;
        Ok(question)
    }
    pub async fn create(db: &DbConn, question_id: Uuid) -> Result<Model> {
        let thread = ActiveModel {
            question_id: sea_orm::ActiveValue::Set(Some(question_id)),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(thread)
    }
    pub async fn update(db: &PgPool, thread_id: Uuid, req: UpdateParams) -> Result<Model> {
        let thread = sqlx::query_as!(
            Model,
            r#"update thread
                            set answer_id = $2
                        where thread.id = $1
                        returning *
                    "#,
            thread_id,
            req.answer_id
        );
        Ok(thread.fetch_one(db).await?)
    }
}
