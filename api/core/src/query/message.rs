use crate::http::{Error, Result};
use entity::{message, message::Entity as Message};
use sea_orm::{DbConn, EntityTrait};
use uuid::Uuid;

pub struct Query {}

impl Query {
    pub async fn find(db: &DbConn, message_id: Uuid) -> Result<message::Model> {
        let message = Message::find_by_id(message_id)
            .one(db)
            .await?
            .ok_or(Error::NotFound("message"))?;

        Ok(message)
    }
}
