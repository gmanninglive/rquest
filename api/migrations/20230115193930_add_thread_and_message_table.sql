BEGIN; 

CREATE TABLE thread (
    thread_id             uuid primary key default uuid_generate_v1mc(),

    created_at            timestamptz not null default now(),
    updated_at            timestamptz
);

SELECT trigger_updated_at('thread');
CREATE TABLE message
(
    message_id            uuid primary key default uuid_generate_v1mc(),
    user_id  uuid         not null references "user" (user_id) on delete cascade,

    thread_id uuid        not null references thread (thread_id) on delete cascade,

    created_at            timestamptz not null default now(),

    updated_at            timestamptz
);

SELECT trigger_updated_at('message');

ALTER TABLE thread
    ADD question_id       uuid references message (message_id),
    ADD answer_id         uuid references message (message_id);

COMMIT;
