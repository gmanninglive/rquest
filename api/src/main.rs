#![feature(async_closure)]
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Router,
};
use rquest::{routes, AppState};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@db:5432".to_string());

    dotenvy::dotenv().unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    let app = Router::new()
        .merge(routes::api_router())
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
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
