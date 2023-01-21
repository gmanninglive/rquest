use uuid::Uuid;
use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "thread")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    pub question_id: Option<Uuid>,
    pub answer_id: Option<Uuid>,

    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::message::Entity",
              from = "Column::QuestionId",
              to = "super::message::Column::Id")]
    Question,
    #[sea_orm(has_one = "super::message::Entity",
              from = "Column::AnswerId",
              to = "super::message::Column::Id",
              )
    ]
    Answer,
}

impl Related<super::message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Question.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

