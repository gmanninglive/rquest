use async_trait::async_trait;
use rquest_core::http::*;
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::{Alias, Expr};
use sea_orm::{QuerySelect, SelectTwo, Set};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};

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

/// Private methods
impl Entity {
    fn find_as_answer(id: Uuid) -> Select<Entity> {
        Self::find().filter(Column::AnswerId.eq(id))
    }
}

pub trait CustomSelectors<E>
where
    E: EntityTrait,
{
    fn with_question(self) -> SelectTwo<Entity, message::Entity>;
    //fn question(self) ->  Select<message::Entity>;
}

impl CustomSelectors<Entity> for Select<Entity> {
    fn with_question(self) -> SelectTwo<Entity, message::Entity> {
        self.join(sea_orm::JoinType::RightJoin, Relation::Question.def())
            .select_also(message::Entity)
    }
}

/// Public interface for querying threads
#[async_trait]
pub trait Query<T> {
    async fn find_by_id(db: &DbConn, id: Uuid) -> Result<T>;
    async fn find_as_answer(dn: &DbConn, id: Uuid) -> Result<T>;
}

#[async_trait]
impl Query<Model> for Entity {
    async fn find_by_id(db: &DbConn, thread_id: Uuid) -> Result<Model> {
        //let t = <Entity as sea_orm::EntityTrait>::find_by_id(thread_id).one_or_nf(db, "thread").await?;
        <Entity as sea_orm::EntityTrait>::find_by_id(thread_id)
            .one_or_nf(db, "thread")
            .await
    }
    async fn find_as_answer(db: &DbConn, as_answer_id: Uuid) -> Result<Model> {
        Entity::find_as_answer(as_answer_id)
            .one_or_nf(db, "thread")
            .await
    }
}

#[derive(Deserialize)]
pub struct UpdateParams {
    answer_id: Uuid,
}

/// Public interface for updating threads
#[async_trait]
pub trait Mutation<T> {
    /// create a new thread, new threads can only be created with a new question message.
    async fn create(db: &DbConn, question_id: Uuid) -> Result<T>;
    async fn update(db: &DbConn, thread_id: Uuid, req: UpdateParams) -> Result<T>;
}

#[async_trait]
impl Mutation<Model> for Entity {
    async fn create(db: &DbConn, question_id: Uuid) -> Result<Model> {
        let thread = ActiveModel {
            question_id: sea_orm::ActiveValue::Set(Some(question_id)),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(thread)
    }
    async fn update(db: &DbConn, thread_id: Uuid, req: UpdateParams) -> Result<Model> {
        let mut thread: ActiveModel = <Entity as Query<Model>>::find_by_id(db, thread_id)
            .await?
            .into();

        thread.answer_id = Set(Some(req.answer_id));

        Ok(thread.update(db).await?)
    }
}
