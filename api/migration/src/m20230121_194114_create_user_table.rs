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
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("uuid_generate_v1mc()"))
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(User::Username)
                            .text()
                            .extra(String::from(r#"collate "case_insensitive""#))
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::Email)
                            .text()
                            .extra(String::from(r#"collate "case_insensitive""#))
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(User::Image).text())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(ColumnDef::new(User::UpdatedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        // create set_updated_at trigger
        let stmt = Statement::from_string(
            manager.get_database_backend(),
            r#"SELECT trigger_updated_at('"user"')"#.to_owned(),
        );
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        // drop set_updated_at trigger
        let stmt = Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TRIGGER IF EXISTS trigger_updated_at ON "user""#.to_owned(),
        );
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Username,
    Email,
    Image,
    CreatedAt,
    UpdatedAt,
}
