pub mod user;
pub mod qanda;

use crate::AppState;
use axum::Router;

pub use crate::http::{Error, ResultExt};

pub fn api_router() -> Router<AppState> {
    Router::new().merge(user::router()).merge(qanda::router())
}
