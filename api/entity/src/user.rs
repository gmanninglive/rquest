use async_trait::async_trait;
use rquest_core::auth;
use rquest_core::http::{Helpers, Result};
use sea_orm::entity::prelude::*;
use sea_orm::entity::*;
use serde::{Deserialize, Serialize};

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
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
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

/// Private methods
impl Entity {
    fn find_by_id(id: Uuid) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }
    fn find_by_email(email: String) -> Select<Entity> {
        Self::find().filter(Column::Email.eq(email))
    }
}

/// Public interface for querying users
#[async_trait]
pub trait Query<T> {
    async fn find_by_id(db: &DbConn, id: Uuid) -> Result<T>;
    async fn find_by_email(db: &DbConn, email: String) -> Result<T>;
    async fn find_all(db: &DbConn) -> Result<Vec<T>>;
}

#[async_trait]
impl Query<Model> for Entity {
    async fn find_by_id(db: &DbConn, user_id: Uuid) -> Result<Model> {
        Entity::find_by_id(user_id).one_or_nf(db, "user").await
    }
    async fn find_by_email(db: &DbConn, email: String) -> Result<Model> {
        Entity::find_by_email(email).one_or_nf(db, "user").await
    }
    async fn find_all(db: &DbConn) -> Result<Vec<Model>> {
        Ok(Entity::find().all(db).await?)
    }
}

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

/// Public interface for updating users
#[async_trait]
pub trait Mutation<T> {
    async fn create(db: &DbConn, req: CreateParams) -> Result<T>;
    async fn update(db: &DbConn, user_id: Uuid, req: UpdateParams) -> Result<T>;
    async fn update_password(db: &DbConn, user_id: Uuid, password: String) -> Result<T>;
    async fn delete(db: &DbConn, user_id: Uuid) -> Result<()>;
}

#[async_trait]
impl Mutation<Model> for Entity {
    async fn create(db: &DbConn, req: CreateParams) -> Result<Model> {
        let user = ActiveModel {
            username: ActiveValue::Set(req.username),
            email: ActiveValue::Set(req.email),
            password_hash: ActiveValue::Set(auth::hash_password(req.password).await?),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(user)
    }
    async fn update(db: &DbConn, user_id: Uuid, req: UpdateParams) -> Result<Model> {
        let mut user: ActiveModel = <Entity as Query<Model>>::find_by_id(db, user_id)
            .await?
            .into();

        match req.username {
            Some(username) => user.username = Set(username),
            None => (),
        };

        match req.email {
            Some(email) => user.email = Set(email),
            None => (),
        };
        match req.image {
            Some(image) => user.image = Set(Some(image)),
            None => (),
        };

        let res = user.update(db).await?;

        Ok(res)
    }
    async fn update_password(db: &DbConn, user_id: Uuid, password: String) -> Result<Model> {
        let mut user: ActiveModel = <Entity as Query<Model>>::find_by_id(db, user_id)
            .await?
            .into();

        user.password_hash = Set(auth::hash_password(password).await?);

        let res = user.update(db).await?;

        Ok(res)
    }
    async fn delete(db: &DbConn, user_id: Uuid) -> Result<()> {
        let user: ActiveModel = <Entity as Query<Model>>::find_by_id(db, user_id)
            .await?
            .into();

        user.delete(db).await?;
        Ok(())
    }
}
