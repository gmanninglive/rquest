use crate::http::{Error, Result};
use entity::{user, user::Entity as User};
use sea_orm::entity::*;
use sea_orm::{DbConn, EntityTrait, QueryFilter};
use uuid::Uuid;

pub struct Query {}

impl Query {
    pub async fn find_all(db: &DbConn) -> Result<Vec<user::Model>> {
        Ok(User::find().all(db).await?)
    }
    pub async fn find(db: &DbConn, user_id: Uuid) -> Result<user::Model> {
        let user = User::find_by_id(user_id)
            .one(db)
            .await?
            .ok_or(Error::NotFound("user"))?;

        Ok(user)
    }
    pub async fn find_by_email(db: &DbConn, email: String) -> Result<user::Model> {
        let user = User::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await?
            .ok_or(Error::NotFound("user"))?;

        Ok(user)
    }
    pub async fn find_by_username(db: &DbConn, username: String) -> Result<user::Model> {
        let user = User::find()
            .filter(user::Column::Username.eq(username))
            .one(db)
            .await?
            .ok_or(Error::NotFound("user"))?;

        Ok(user)
    }
}
