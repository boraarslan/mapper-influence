-- Add up migration script here
CREATE TABLE IF NOT EXISTS users(
    id BIGSERIAL PRIMARY KEY,
    user_name TEXT NOT NULL,
    profile_picture TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS user_profiles(
    user_id BIGSERIAL REFERENCES users(id),
    bio TEXT,
    featured_maps JSON
);

CREATE TABLE IF NOT EXISTS users_osu_data(
    user_id BIGSERIAL REFERENCES users(id),
    ranked_count INT NOT NULL DEFAULT 0,
    loved_count INT NOT NULL DEFAULT 0,
    nominated_count INT NOT NULL DEFAULT 0,
    graveyard_count INT NOT NULL DEFAULT 0,
    guest_count INT NOT NULL DEFAULT 0,
    modified_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS influences(
    from_id BIGSERIAL REFERENCES users(id),
    to_id BIGSERIAL REFERENCES users(id),
    influence_level INT NOT NULL,
    info TEXT,
    CONSTRAINT pk_influence PRIMARY KEY (from_id, to_id)
);
