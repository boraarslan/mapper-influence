-- DROP down migration script here

ALTER TABLE users DROP COLUMN IF EXISTS created_at;
ALTER TABLE users DROP COLUMN IF EXISTS modified_at;

ALTER TABLE user_profiles DROP COLUMN IF EXISTS created_at;
ALTER TABLE user_profiles DROP COLUMN IF EXISTS modified_at;

ALTER TABLE users_osu_data DROP COLUMN IF EXISTS created_at;

ALTER TABLE influences DROP COLUMN IF EXISTS created_at;
ALTER TABLE influences DROP COLUMN IF EXISTS modified_at;

ALTER TABLE user_osu_maps DROP COLUMN IF EXISTS created_at;
ALTER TABLE user_osu_maps DROP COLUMN IF EXISTS modified_at;
