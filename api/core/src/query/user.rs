use crate::http::{Error, Result};
pub use entity::{user, user::Column::Email, user::Entity as User};
use sea_orm::sea_query::SimpleExpr;
use sea_orm::{DbConn, EntityTrait, QueryFilter};
use uuid::Uuid;

pub struct UserQuery {}

impl UserQuery {
    pub async fn find_all(db: &DbConn) -> Result<Vec<user::Model>> {
        Ok(User::find().all(db).await?)
    }
    pub async fn find(db: &DbConn, user_id: Uuid) -> Result<user::Model> {
        Ok(User::find_by_id(user_id)
            .one(db)
            .await?
            .ok_or(Error::NotFound("user"))?)
    }
    pub async fn find_by(db: &DbConn, condition: SimpleExpr) -> Result<user::Model> {
        let user = User::find()
            .filter(condition)
            .one(db)
            .await?
            .ok_or(Error::NotFound("user"))?;

        Ok(user)
    }
}
