use rquest_core::auth;
use rquest_core::http::*;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "Text", unique)]
    pub username: String,
    #[sea_orm(column_type = "Text", unique)]
    pub email: String,
    #[sea_orm(column_type = "Text")]
    #[serde(skip_deserializing, skip_serializing)]
    pub password_hash: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub image: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::message::Entity")]
    Message,
    #[sea_orm(has_many = "super::session::Entity")]
    Session,
}

impl Related<super::message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Message.def()
    }
}

impl Related<super::session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Session.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(serde::Deserialize, Default, PartialEq, Eq)]
#[serde(default)] // fill in any missing fields with `..UpdateUser::default()`
pub struct UpdateParams {
    email: Option<String>,
    image: Option<String>,
    password: Option<String>,
    username: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateParams {
    username: String,
    image: Option<String>,
    email: String,
    password: String,
}

impl Entity {
    pub async fn find_by_id(db: &PgPool, user_id: Uuid) -> Result<Model> {
        Ok(sqlx::query_as!(
            Model,
            r#"
                select * from "user"
                where id = $1"#,
            user_id
        )
        .fetch_one(db)
        .await?)
    }
    pub async fn find_by_email(db: &PgPool, email: String) -> Result<Model> {
        Ok(sqlx::query_as!(
            Model,
            r#"
                select * from "user"
                where email = $1"#,
            email
        )
        .fetch_one(db)
        .await?)
    }
    pub async fn find_all(db: &PgPool) -> Result<Vec<Model>> {
        Ok(sqlx::query_as!(Model, r#"select * from "user""#)
            .fetch_all(db)
            .await?)
    }
    pub async fn create(db: &PgPool, req: CreateParams) -> Result<Model> {
        let user = sqlx::query_as!(
            Model,
            r#"
            insert into "user" (username, email, image, password_hash)
            values ($1, $2, $3, $4)
            returning * 
        "#,
            req.username,
            req.email,
            req.image,
            auth::hash_password(req.password).await?
        )
        .fetch_one(db)
        .await
        .on_constraint("user_username_key", |_| {
            Error::unprocessable_entity([("username", "username taken")])
        })
        .on_constraint("user_email_key", |_| {
            Error::unprocessable_entity([("email", "email taken")])
        })?;

        Ok(user)
    }
    pub async fn update(db: &PgPool, user_id: Uuid, req: UpdateParams) -> Result<Model> {
        let password_hash = if let Some(password) = req.password {
            Some(auth::hash_password(password).await?)
        } else {
            None
        };

        let user = sqlx::query_as!(
            Model,
            r#"
            update "user"
            set email = coalesce($1, "user".email),
                username = coalesce($2, "user".username),
                password_hash = coalesce($3, "user".password_hash),
                image = coalesce($4, "user".image)
            where id = $5
            returning * 
        "#,
            req.email,
            req.username,
            password_hash,
            req.image,
            user_id
        )
        .fetch_one(db)
        .await
        .on_constraint("user_username_key", |_| {
            Error::unprocessable_entity([("username", "username taken")])
        })
        .on_constraint("user_email_key", |_| {
            Error::unprocessable_entity([("email", "email taken")])
        })?;

        Ok(user)
    }
    pub async fn delete(db: &PgPool, user_id: Uuid) -> Result<()> {
        sqlx::query!(
            r#"
                        delete from "user"
                        where id = $1
                        "#,
            user_id
        )
        .execute(db)
        .await?;

        Ok(())
    }
}
