pub mod auth;
pub mod http;

pub use sea_orm;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    #[allow(dead_code)]
    pub db: PgPool,
    pub hmac_key: String,
}
