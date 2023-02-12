//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.7

use async_trait::async_trait;
use rquest_core::http::*;
use sea_orm::{entity::prelude::*, SelectTwoMany};

use crate::{message, thread};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "session")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub title: Option<String>,
    pub host_id: Option<Uuid>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub scheduled_for: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::thread::Entity")]
    Thread,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::HostId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    User,
}

impl Related<super::thread::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Thread.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

trait CustomSelectors<E>where
    E: EntityTrait {
    fn with_threads(self) -> SelectTwoMany<Entity, thread::Entity>;
    //fn with_questions(self) -> SelectTwoMany<Entity, message::Entity>;
}

impl CustomSelectors<Entity> for Select<Entity> {
   fn with_threads(self) -> SelectTwoMany<Entity, thread::Entity> {
        self.find_with_related(thread::Entity)
    }
    //fn with_questions(self) -> SelectTwoMany<Entity, message::Entity> {
    //}
}

impl Model {
    pub async fn threads(&self, db: &DbConn) -> Result<Vec<thread::Model>> {
        Ok(self.find_related(thread::Entity).all(db).await?)
    }
}

#[async_trait]
trait Query {
    async fn find_by_id(db: &DbConn, id: Uuid) -> Result<Model>;
}
#[async_trait]
impl Query for Entity {
    async fn find_by_id(db: &DbConn, id: Uuid) -> Result<Model> {
        <Entity as sea_orm::EntityTrait>::find_by_id(id).one_or_nf(db, "session").await
    }
}
