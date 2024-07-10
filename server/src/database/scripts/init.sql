create schema default_idx;

set search_path to default_idx;

create table default_idx.log(
    id serial primary key,
    log_type varchar(255),
    log_message varchar(255),
    log_timestamp timestamp default current_timestamp,
    log_attributes jsonb[]
);

