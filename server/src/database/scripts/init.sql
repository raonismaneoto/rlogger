create table log(
    id serial primary key,
    log_type varchar(255),
    log_message varchar(255),
    log_timestamp timestamp default current_timestamp,
    log_attributes json[],
    log_index varchar(255)
)