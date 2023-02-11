pub use sea_orm_migration::prelude::*;

mod m20220101_000001_setup;
mod m20230121_194114_create_user_table;
mod m20230121_220247_create_message_table;
mod m20230122_001116_create_thread_table;
mod m20230122_002511_add_thread_relation_to_message;
mod m20230206_214835_add_session_table;
mod m20230211_131958_add_session_fk_to_thread;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_setup::Migration),
            Box::new(m20230121_194114_create_user_table::Migration),
            Box::new(m20230121_220247_create_message_table::Migration),
            Box::new(m20230122_001116_create_thread_table::Migration),
            Box::new(m20230122_002511_add_thread_relation_to_message::Migration),
            Box::new(m20230206_214835_add_session_table::Migration),
            Box::new(m20230211_131958_add_session_fk_to_thread::Migration),
        ]
    }
}
