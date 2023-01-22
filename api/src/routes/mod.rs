pub mod auth;
pub mod message;
pub mod qanda;
pub mod user;

use crate::AppState;
use axum::Router;

pub use crate::http::{Error, ResultExt};

pub fn api_router() -> Router<AppState> {
    Router::new()
        .merge(user::router())
        .merge(qanda::router())
        .merge(message::router())
        .merge(auth::router())
}
