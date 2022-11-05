-- Add up migration script here
create table users(
    id bigserial primary key,
    user_name text not null,
    profile_picture text not null,
    bio text
);

create table influences(
    from_id bigserial references users(id),
    to_id bigserial references users(id),
    influence_level int,
    info text,
    constraint pk_influence primary key (from_id, to_id)
);
