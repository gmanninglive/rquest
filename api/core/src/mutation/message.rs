use crate::http::Result;
use entity::message;
use sea_orm::DbConn;
use sea_orm::{ActiveModelTrait, ActiveValue};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct NewMessageParams {
    user_id: Uuid,
    text: String,
    as_question_thread_id: Option<Uuid>,
    as_answer_thread_id: Option<Uuid>,
    publish: Option<bool>,
}

pub struct Mutation {}

impl Mutation {
    pub async fn new(db: &DbConn, req: NewMessageParams) -> Result<message::Model> {
        let message = message::ActiveModel {
            user_id: ActiveValue::Set(Some(req.user_id)),
            text: ActiveValue::Set(Some(req.text)),
            state: match req.publish {
                Some(publish) => {
                    if publish == true {
                        ActiveValue::Set(message::MessageState::Posted)
                    } else {
                        ActiveValue::NotSet
                    }
                }
                _ => ActiveValue::NotSet,
            },
            thread_answer_id: match req.as_answer_thread_id {
                Some(thread_id) => ActiveValue::Set(Some(thread_id)),
                None => ActiveValue::NotSet,
            },
            thread_question_id: match req.as_question_thread_id {
                Some(thread_id) => ActiveValue::Set(Some(thread_id)),
                None => ActiveValue::NotSet,
            },
            ..Default::default()
        };

        Ok(message.insert(db).await?)
    }
}
