use async_trait::async_trait;
use rquest_core::http::*;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "message")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    #[sea_orm(column_type = "Text", nullable)]
    pub text: Option<String>,
    pub state: MessageState,
    pub created_at: DateTimeWithTimeZone,
    pub posted_at: Option<DateTimeWithTimeZone>,
    pub updated_at: Option<DateTimeWithTimeZone>,
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
    Thread2,
    #[sea_orm(
        belongs_to = "super::thread::Entity",
        from = "Column::ThreadQuestionId",
        to = "super::thread::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    Thread1,
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
        Relation::Thread1.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, Eq, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "i16", db_type = "Integer")]
pub enum MessageState {
    Deleted = -1,
    Draft = 0,
    Posted = 1,
}

impl Entity {
    pub fn find_by_id(id: Uuid) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }
}

#[async_trait]
pub trait Query<T> {
    async fn find_by_id(db: &DbConn, id: Uuid) -> Result<T>;
}

#[async_trait]
impl Query<Model> for Entity {
    async fn find_by_id(db: &DbConn, message_id: Uuid) -> Result<Model> {
        let message = Entity::find_by_id(message_id)
            .one(db)
            .await?
            .ok_or(Error::NotFound("message"))?;

        Ok(message)
    }
}

#[derive(Deserialize)]
pub struct CreateParams {
    pub user_id: Uuid,
    pub text: String,
    pub as_question_thread_id: Option<Uuid>,
    pub as_answer_thread_id: Option<Uuid>,
    pub publish: Option<bool>,
}

#[async_trait]
pub trait Mutation<T> {
    async fn create(db: &DbConn, req: CreateParams) -> Result<T>;
    //async fn update(db: &DbConn, user_id: Uuid, req: UpdateParams) -> Result<T>;
    //async fn delete(db: &DbConn, user_id: Uuid) -> Result<()>;
}

#[async_trait]
impl Mutation<Model> for Entity {
    async fn create(db: &DbConn, req: CreateParams) -> Result<Model> {
        let message = ActiveModel {
            user_id: ActiveValue::Set(Some(req.user_id)),
            text: ActiveValue::Set(Some(req.text)),
            state: match req.publish {
                Some(publish) => {
                    if publish == true {
                        ActiveValue::Set(MessageState::Posted)
                    } else {
                        ActiveValue::NotSet
                    }
                }
                _ => ActiveValue::NotSet,
            },
            thread_answer_id: match req.as_answer_thread_id {
                Some(thread_id) => ActiveValue::Set(Some(thread_id)),
                None => ActiveValue::NotSet,
            },
            thread_question_id: match req.as_question_thread_id {
                Some(thread_id) => ActiveValue::Set(Some(thread_id)),
                None => ActiveValue::NotSet,
            },
            ..Default::default()
        };

        Ok(message.insert(db).await?)
    }
}
