-- sql schema for openmind application

-- add trigram extension for full text search
create extension if not exists pg_trgm;

-- add uuid extension for generating uuids
create extension if not exists "uuid-ossp";

-- add ltree extension for storing hierarchical data
create extension if not exists ltree;

-- add thought_variation enum
create type thought_variation as enum ('thought', 'question');

-- create table for note
create table note (
    note_id uuid primary key,
    imported_at timestamp not null default now(),
    scribe_id uuid not null references scribe(scribe_id),
    project_id uuid not null references project(project_id),
    content text not null
);

-- create table for thought
create table thought (
    thought_id uuid primary key,
    parent_id uuid references thought(thought_id),
    refuted_by uuid references thought(thought_id),
    created_at timestamp not null default now(),
    scribe_id uuid not null references scribe(scribe_id),
    project_id uuid not null references project(project_id),
    variation thought_variation not null,
    content text not null,
    tags text[],
    categories ltree[],
    media jsonb,
    references jsonb[],
    links uuid[]
);

-- create table for project
create table project (
    project_id uuid primary key,
    universe_id uuid not null references universe(universe_id),
    created_at timestamp not null default now(),
    project_name text not null,
    locked boolean not null default false
);

-- create table for universe
create table universe (
    universe_id uuid primary key,
    organization_id uuid not null references organization(organization_id),
    is_private boolean not null default false,
);

-- create table for organization
create table organization (
    organization_id uuid primary key,
    organization_name text not null
);

-- create table for scribe
create table scribe (
    scribe_id int primary key,
    scribe_name text not null,
    created_at timestamp not null default now(),
);

-- create indexes for full text search
create index idx_thought_content on thought using gin (content gin_trgm_ops);