use rquest_core::http::*;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::PgPool;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Deserialize,
    Serialize,
    sqlx::FromRow,
    sqlx::Type,
)]
pub struct Model {
    pub id: Uuid,
    pub title: Option<String>,
    pub host_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub scheduled_for: DateTime<Utc>,
}

impl Model {
    pub async fn find_by_id(db: &PgPool, event_id: Uuid) -> Result<Model> {
        Ok(sqlx::query_as!(Model, 
            r#"select * from event where id = $1"#, event_id).fetch_one(db).await?)
    }
}
