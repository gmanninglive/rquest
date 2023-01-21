use sea_orm_migration::sea_orm::ConnectionTrait;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
        CREATE EXTENSION if not exists "uuid-ossp";
        CREATE OR REPLACE function set_updated_at()
            returns trigger as
        $$
        begin
            NEW.updated_at = now();
            return NEW;
        end;
        $$ language plpgsql;

        CREATE OR REPLACE function trigger_updated_at(tablename regclass)
            returns void as
        $$
        begin
            execute format('CREATE TRIGGER set_updated_at
                BEFORE UPDATE
                ON %s
                FOR EACH ROW
                WHEN (OLD is distinct from NEW)
            EXECUTE FUNCTION set_updated_at();', tablename);
        end;
        $$ language plpgsql;

        -- Finally, this is a text collation that sorts text case-insensitively, useful for `UNIQUE` indexes
        CREATE collation case_insensitive (provider = icu, locale = 'und-u-ks-level2', deterministic = false);
        "#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

