use entity::{message, session, thread, user};
use migration::sea_orm::prelude::{DateTimeUtc, DateTimeWithTimeZone};
use migration::sea_orm::ActiveModelTrait;
use rquest_core::auth;
use sea_orm::entity::ActiveValue::Set;
use sea_orm::EntityTrait;
use sea_orm::{Database, DbConn};
use sea_orm_migration::prelude::*;
use std::env;

#[async_std::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1] == "seed".to_owned() {
        dotenvy::dotenv().unwrap();

        let db_connection_str = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@db:5432".to_string());

        let db = Database::connect(db_connection_str).await.unwrap();

        seed(&db).await.unwrap();
    } else {
        cli::run_cli(migration::Migrator).await;
    }
}

async fn seed(db: &DbConn) -> Result<(), DbErr> {
    let password = auth::hash_password("password".to_owned()).await.unwrap();
    let mut users: Vec<user::ActiveModel> = vec![];
    for n in 1..50 {
        users.push(user::ActiveModel {
            username: Set(format!("user-{}", n)),
            email: Set(format!("user-{}@example.com", n)),
            password_hash: Set(password.clone()),
            ..Default::default()
        })
    }

    user::Entity::insert_many(users).exec(db).await?;

    let users = user::Entity::find().all(db).await?;

    let session = session::ActiveModel {
        host_id: Set(Some(users[0].id)),
        title: Set(Some("First Q&A!".to_owned())),
        scheduled_for: Set(DateTimeUtc::default()),
        ..Default::default()
    }
    .insert(db)
    .await?;

    for user in users[1..].iter() {
        let message = message::ActiveModel {
            text: Set(Some("I have a question".to_owned())),
            user_id: Set(Some(user.id)),
            state: Set(1),
            ..Default::default()
        }
        .insert(db)
        .await?;

        thread::ActiveModel {
            session_id: Set(Some(session.id)),
            question_id: Set(Some(message.id)),
            ..Default::default()
        }
        .insert(db)
        .await?;
    }
    Ok(())
}
