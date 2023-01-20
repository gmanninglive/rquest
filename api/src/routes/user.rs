use crate::{http::*, AppState};
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    username: String,
    email: String,
    image: Option<String>,
}

async fn create_user(State(state): State<AppState>, Json(req): Json<User>) -> Result<Json<User>> {
    //sqlx::query_scalar!(
        //r#"insert into "user" (username, email) values ($1, $2) returning user_id"#,
        //req.username,
        //req.email,
    //)
    //.fetch_one(&state.db)
    //.await
    //.on_constraint("user_username_key", |_| {
        //Error::unprocessable_entity([("username", "username taken")])
    //})
    //.on_constraint("user_email_key", |_| {
        //Error::unprocessable_entity([("email", "email taken")])
    //})?;

    Ok(Json(User {
        email: req.email,
        username: req.username,
        image: None,
    }))
}

async fn index(State(state): State<AppState>) -> Result<Json<Vec<User>>> {
    //let users = sqlx::query_as::<_, User>(r#"SELECT * FROM "user""#)
        //.fetch_all(&state.db)
        //.await?;
    let users: Vec<User> = vec![];

    Ok(Json(users))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users", get(index))
        .route("/user/create", post(create_user))
}
