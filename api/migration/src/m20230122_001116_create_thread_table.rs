use super::m20230121_220247_create_message_table::Message;
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
                    .table(Thread::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Thread::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("uuid_generate_v1mc()"))
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Thread::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(Thread::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .to_owned(),
            )
            .await?;

        let stmt = Statement::from_string(
            manager.get_database_backend(),
            r#"SELECT trigger_updated_at('"thread"')"#.to_owned(),
        );

        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Thread::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Thread {
    Table,
    Id,
    QuestionId,
    AnswerId,
    CreatedAt,
    UpdatedAt,
}
