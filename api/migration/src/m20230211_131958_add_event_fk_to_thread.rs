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
                    .add_column(ColumnDef::new(Thread::EventId).uuid())
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-event-id")
                    .from(Thread::Table, Thread::EventId)
                    .to(Event::Table, Event::Id)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("index_thread_on_event_id")
                    .table(Thread::Table)
                    .col(Thread::EventId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Thread::Table)
                    .drop_foreign_key(Alias::new("fk-event-id"))
                    .drop_column(Thread::EventId)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum Thread {
    Table,
    EventId,
}

#[derive(Iden)]
enum Event {
    Table,
    Id,
}
