use super::m20230121_194114_create_user_table::User;
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
                    .table(Message::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Message::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("uuid_generate_v1mc()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Message::UserId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-id")
                            .from(Message::Table, Message::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .col(ColumnDef::new(Message::Text).text())
                    .col(
                        ColumnDef::new(Message::State)
                            .small_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Message::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(ColumnDef::new(Message::PostedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Message::UpdatedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        let stmts = [
            r#"
        CREATE OR REPLACE function set_posted_at()
            returns trigger as
        $$
        begin
            NEW.posted_at = now();

            return NEW;
        end;
        $$ language plpgsql;
        "#,
            r#"
           CREATE TRIGGER trigger_posted_at 
                BEFORE UPDATE
                ON "message" 
                FOR EACH ROW
                WHEN (NEW.state = 1)
           EXECUTE FUNCTION set_posted_at();
        "#,
            r#"SELECT trigger_updated_at('"message"')"#,
        ]
        .map(|sql| Statement::from_string(manager.get_database_backend(), sql.to_owned()));

        for stmt in stmts {
            manager.get_connection().execute(stmt).await.map(|_| ())?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Message::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Message {
    Table,
    Id,
    UserId,
    Text,
    State,
    PostedAt,
    CreatedAt,
    UpdatedAt,
}
