use async_trait::async_trait;
use rquest_core::http::*;
use sea_orm::{entity::prelude::*, SelectTwoMany};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};

use crate::{message, thread};

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    Eq,
    Deserialize,
    Serialize,
    sqlx::FromRow,
    sqlx::Type,
)]
#[sea_orm(table_name = "session")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub title: Option<String>,
    pub host_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub scheduled_for: DateTime<Utc>,
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

impl Entity {
    fn with_threads() -> SelectTwoMany<Entity, thread::Entity> {
        Self::find().find_with_related(thread::Entity)
    }
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
        <Entity as sea_orm::EntityTrait>::find_by_id(id)
            .one_or_nf(db, "session")
            .await
    }
}
