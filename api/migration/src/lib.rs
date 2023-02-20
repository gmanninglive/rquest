pub use sea_orm_migration::prelude::*;

mod m20220101_000001_setup;
mod m20230121_194114_create_user_table;
mod m20230121_220247_create_message_table;
mod m20230122_001116_create_thread_table;
mod m20230206_214835_add_event_table;
mod m20230211_131958_add_event_fk_to_thread;
mod m20230219_225446_create_question_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_setup::Migration),
            Box::new(m20230121_194114_create_user_table::Migration),
            Box::new(m20230121_220247_create_message_table::Migration),
            Box::new(m20230122_001116_create_thread_table::Migration),
            Box::new(m20230206_214835_add_event_table::Migration),
            Box::new(m20230211_131958_add_event_fk_to_thread::Migration),
            Box::new(m20230219_225446_create_question_table::Migration),
        ]
    }
}
