use async_trait::async_trait;
use rquest_core::http::*;
use sea_orm::entity::prelude::*;
use sea_orm::Set;
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "thread")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub question_id: Option<Uuid>,
    pub answer_id: Option<Uuid>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
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
    Message2,
    #[sea_orm(
        has_one = "super::message::Entity",
        from = "Column::QuestionId",
        to = "super::message::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    Message1,
}

impl ActiveModelBehavior for ActiveModel {}

/// Private methods
impl Entity {
    fn find_by_id(id: Uuid) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }
    fn find_as_answer(id: Uuid) -> Select<Entity> {
        Self::find().filter(Column::AnswerId.eq(id))
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
        Entity::find_by_id(thread_id).one_or_nf(db, "thread").await
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
