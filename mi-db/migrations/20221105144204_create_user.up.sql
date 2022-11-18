-- Add up migration script here
create table if not exists users(
    id bigserial primary key,
    user_name text not null,
    profile_picture text not null,
    bio text
);

create table if not exists influences(
    from_id bigserial references users(id),
    to_id bigserial references users(id),
    influence_level int not null,
    info text,
    constraint pk_influence primary key (from_id, to_id)
);
