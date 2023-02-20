use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // create table with id, message id, thread id
        manager
            .create_table(
                Table::create()
                    .table(Question::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Question::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("uuid_generate_v1mc()"))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Question::MessageId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-question-id")
                            .from(Question::Table, Question::MessageId)
                            .to(Message::Table, Message::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Question::ThreadId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-thread-id")
                            .from(Question::Table, Question::ThreadId)
                            .to(Thread::Table, Thread::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // add question id column and foreign key to thread table
        manager
            .alter_table(
                Table::alter()
                    .table(Thread::Table)
                    .add_column(ColumnDef::new(Thread::QuestionId).uuid())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-question-id")
                    .from(Thread::Table, Thread::QuestionId)
                    .to(Question::Table, Question::Id)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await?;

        // index thread id on question
        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("index_question_thread_id")
                    .table(Question::Table)
                    .col(Question::ThreadId)
                    .to_owned(),
            )
            .await?;

        // index message id on question
        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("index_question_message_id")
                    .table(Question::Table)
                    .col(Question::MessageId)
                    .to_owned(),
            )
            .await?;

        // index question id on thread
        manager
            .create_index(
                IndexCreateStatement::new()
                    .name("index_thread_question_id")
                    .table(Thread::Table)
                    .col(Thread::QuestionId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Question::Table).to_owned())
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Thread::Table)
                    .drop_column(Thread::QuestionId)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum Question {
    Table,
    Id,
    MessageId,
    ThreadId,
}

#[derive(Iden)]
enum Thread {
    Table,
    Id,
    QuestionId,
}

#[derive(Iden)]
enum Message {
    Table,
    Id,
}
