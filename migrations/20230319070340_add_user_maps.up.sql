-- Add up migration script here
CREATE TABLE IF NOT EXISTS user_osu_maps(
    user_id BIGSERIAL REFERENCES users(id),
    mapsets JSON
)
