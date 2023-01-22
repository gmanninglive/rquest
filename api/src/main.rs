mod http;
mod routes;
mod types;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Router,
};
use sea_orm::{Database, DatabaseConnection};
use std::net::SocketAddr;

#[derive(Clone)]
pub struct AppState {
    #[allow(dead_code)]
    db: DatabaseConnection,
    hmac_key: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@db:5432".to_string());

    dotenvy::dotenv().unwrap();

    let pool: DatabaseConnection = Database::connect(&db_connection_str).await?;

    let app = Router::new()
        .merge(routes::api_router())
        .with_state(AppState {
            db: pool,
            hmac_key: std::env::var("HMAC_KEY").unwrap(),
        });

    let addr = SocketAddr::from(([0, 0, 0, 0], 3030));
    println!("listening on address: {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

pub struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
