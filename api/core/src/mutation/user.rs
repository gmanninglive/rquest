use crate::auth::hash_password;
use crate::http::Result;
use crate::query::user::UserQuery;
use entity::user;
use sea_orm::entity::*;
use sea_orm::DbConn;
use serde::Deserialize;
use uuid::Uuid;

pub struct UserMutation {}

#[derive(Deserialize)]
pub struct UpdateParams {
    username: Option<String>,
    email: Option<String>,
    image: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateParams {
    username: String,
    email: String,
    password: String,
}

impl UserMutation {
    pub async fn create(db: &DbConn, req: CreateParams) -> Result<user::Model> {
        let user = user::ActiveModel {
            username: ActiveValue::Set(req.username),
            email: ActiveValue::Set(req.email),
            password_hash: ActiveValue::Set(hash_password(req.password).await?),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(user)
    }
    pub async fn update(db: &DbConn, user_id: Uuid, req: UpdateParams) -> Result<user::Model> {
        let user = UserQuery::find(db, user_id).await?;

        let res = user::ActiveModel {
            id: ActiveValue::Unchanged(user.id),
            username: match req.username {
                Some(username) => Set(username),
                None => ActiveValue::Unchanged(user.username),
            },
            email: match req.email {
                Some(email) => Set(email),
                None => ActiveValue::Unchanged(user.email),
            },
            image: match req.image {
                Some(image) => Set(Some(image)),
                None => ActiveValue::Unchanged(user.image),
            },
            ..Default::default()
        }
        .update(db)
        .await?;

        Ok(res)
    }
    pub async fn update_password(
        db: &DbConn,
        user_id: Uuid,
        password: String,
    ) -> Result<user::Model> {
        let user = UserQuery::find(db, user_id).await?;

        let res = user::ActiveModel {
            id: ActiveValue::Unchanged(user.id),
            password_hash: ActiveValue::Set(hash_password(password).await?),
            ..Default::default()
        }
        .update(db)
        .await?;

        Ok(res)
    }
    pub async fn delete(db: &DbConn, user_id: Uuid) -> Result<()> {
        let user: user::ActiveModel = UserQuery::find(db, user_id).await.map(Into::into)?;

        user.delete(db).await?;
        Ok(())
    }
}
