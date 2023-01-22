use crate::http::extractor::AuthUser;
use crate::{http::*, AppState};
use axum::{
    extract::{Path, State},
    routing::{delete, get, patch, post},
    Json, Router,
};
use entity::{
    message::{self, Entity as Message},
    user,
    user::Entity as User,
};
use sea_orm::entity::*;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
struct NewMessageParams {
    text: String,
    publish: Option<bool>,
}

async fn new_message(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(params): Json<NewMessageParams>,
) -> Result<Json<message::Model>> {
    let user = User::find()
        .filter(user::Column::Id.eq(auth_user.user_id))
        .one(&state.db)
        .await?;

    match user {
        Some(user) => {
            let message = message::ActiveModel {
                user_id: ActiveValue::Set(Some(user.id)),
                text: ActiveValue::Set(Some(params.text)),
                state: match params.publish {
                    Some(publish) => {
                        if publish == true {
                            ActiveValue::Set(message::MessageState::Posted)
                        } else {
                            ActiveValue::NotSet
                        }
                    }
                    _ => ActiveValue::NotSet,
                },
                ..Default::default()
            };

            let res = message.insert(&state.db).await?;
            Ok(Json(res))
        }
        None => Err(Error::NotFound("user")),
    }
}

async fn get_message(State(state): State<AppState>, Path(message_id): Path<Uuid>) -> Result<Json<message::Model>> {
    let message = Message::find_by_id(message_id).one(&state.db).await?;

    match message {
        Some(message) => Ok(Json(message)),
        None => Err(Error::NotFound("message"))
    }
}


pub fn router() -> Router<AppState> {
    Router::new().route("/message/new", post(new_message)).route("/message/:id", get(get_message))
}
