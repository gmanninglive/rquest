use rquest_core::http::*;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::PgPool;

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
    pub async fn find_by_id(db: &PgPool, session_id: Uuid) -> Result<Model> {
        Ok(sqlx::query_as!(Model, 
                    r#"select * from session where id = $1"#, session_id).fetch_one(db).await?)
    }
}
