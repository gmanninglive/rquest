use super::m20230122_001116_create_thread_table::Thread;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Message::Table)
                    .add_column(ColumnDef::new(Message::ThreadQuestionId).uuid())
                    .add_column(ColumnDef::new(Message::ThreadAnswerId).uuid())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-thread-question-id")
                    .from(Message::Table, Message::ThreadQuestionId)
                    .to(Thread::Table, Thread::Id)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-thread-answer-id")
                    .from(Message::Table, Message::ThreadAnswerId)
                    .to(Thread::Table, Thread::Id)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Message::Table)
                    .drop_column(Message::ThreadQuestionId)
                    .drop_column(Message::ThreadAnswerId)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(Debug, Iden)]
enum Message {
    Table,
    ThreadQuestionId,
    ThreadAnswerId,
}
