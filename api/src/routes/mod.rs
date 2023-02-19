pub mod auth;
pub mod message;
pub mod test;
pub mod thread;
pub mod user;

use crate::AppState;
use axum::Router;

pub use crate::http::{Error, ResultExt};

pub fn api_router() -> Router<AppState> {
    Router::new()
        .merge(user::router())
        .merge(thread::router())
        .merge(message::router())
        .merge(auth::router())
}
