use sea_orm_migration::sea_orm::ConnectionTrait;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmt1 = r#"
        CREATE EXTENSION if not exists "uuid-ossp";
        "#;

        // create a function to set default timestamp to now()
        let stmt2 = r#"
        CREATE OR REPLACE function set_updated_at()
            returns trigger as
        $$
        begin
            NEW.updated_at = now();
            return NEW;
        end;
        $$ language plpgsql;
        "#;
        
        // create trigger generation helper for updated_at column 
        // usage `SELECT trigger_updated_at('<table_name>')
        let stmt3 = r#"
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
        "#;
        // Finally, this is a text collation that sorts text case-insensitively, useful for `UNIQUE` indexes
        let stmt4 = r#"
        CREATE collation case_insensitive (provider = icu, locale = 'und-u-ks-level2', deterministic = false);
        "#;
        let stmts = [stmt1, stmt2, stmt3, stmt4].map(|sql| {
             Statement::from_string(manager.get_database_backend(), sql.to_owned())
        });

        for stmt in stmts {
            manager.get_connection().execute(stmt).await.map(|_| ())?;
        }
        Ok(())
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

