create table note (
    note_id uuid primary key,
    imported_at timestamp not null default now(),
    scribe_id uuid not null references scribe(scribe_id),
    project_id uuid not null references project(project_id),
    content text not null
);

