pub mod auth;
pub mod http;

pub use sea_orm;

#[derive(Clone)]
pub struct AppState {
    #[allow(dead_code)]
    pub db: sea_orm::DbConn,
    pub hmac_key: String,
}