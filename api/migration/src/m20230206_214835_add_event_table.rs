use sea_orm::Statement;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::ConnectionTrait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Event::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Event::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("uuid_generate_v1mc()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Event::Title).string())
                    .col(ColumnDef::new(Event::HostId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-host-id")
                            .from(Event::Table, Event::HostId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .col(
                        ColumnDef::new(Event::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(Event::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(Event::ScheduledFor)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // create set_updated_at trigger
        let stmt = Statement::from_string(
            manager.get_database_backend(),
            r#"SELECT trigger_updated_at('"event"')"#.to_owned(),
        );
        manager.get_connection().execute(stmt).await.map(|_| (()))
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Event::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Event {
    Table,
    Id,
    Title,
    HostId,
    CreatedAt,
    UpdatedAt,
    ScheduledFor,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
}
