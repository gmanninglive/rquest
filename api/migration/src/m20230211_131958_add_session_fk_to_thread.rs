use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Thread::Table)
                    .add_column(ColumnDef::new(Thread::SessionId).uuid())
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-session-id")
                    .from(Thread::Table, Thread::SessionId)
                    .to(Session::Table, Session::Id)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("index_thread_on_session_id")
                    .table(Thread::Table)
                    .col(Thread::SessionId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Thread::Table)
                    .drop_foreign_key(Alias::new("fk-session-id"))
                    .drop_column(Thread::SessionId)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum Thread {
    Table,
    SessionId,
}

#[derive(Iden)]
enum Session {
    Table,
    Id,
}
