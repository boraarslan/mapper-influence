-- Add up migration script here
create table if not exists users(
    id bigserial primary key,
    user_name text not null,
    profile_picture text not null
);

create table if not exists user_profiles(
    user_id bigserial references users(id),
    bio text,
    featured_maps json
);

create table if not exists users_osu_data(
    user_id bigserial references users(id),
    ranked_count int not null default 0,
    loved_count int not null default 0,
    nominated_count int not null default 0,
    graveyard_count int not null default 0,
    guest_count int not null default 0,
    modified_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

create table if not exists influences(
    from_id bigserial references users(id),
    to_id bigserial references users(id),
    influence_level int not null,
    info text,
    constraint pk_influence primary key (from_id, to_id)
);
