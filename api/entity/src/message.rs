use uuid::Uuid;
use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum MessageState {
    #[sea_orm(num_value = -1)]
    Deleted,
    #[sea_orm(num_value = 0)]
    Draft,
    #[sea_orm(num_value = 1)]
    Published,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "message")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub user_id: Option<Uuid>,


    pub state: MessageState,
    pub text: String,

    pub as_question_thread_id: Option<Uuid>,
    pub as_answer_thread_id: Option<Uuid>,

    pub posted_at: DateTimeWithTimeZone,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::user::Entity",
              from = "Column::UserId",
              to = "super::user::Column::Id")]
    User,
    #[sea_orm(belongs_to = "super::thread::Entity",
              from = "Column::AsAnswerThreadId",
              to = "super::thread::Column::AnswerId")]
    AsAnswer,
    #[sea_orm(belongs_to = "super::thread::Entity",
              from = "Column::AsQuestionThreadId",
              to = "super::thread::Column::QuestionId")]
    AsQuestion,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::thread::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AsAnswer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

