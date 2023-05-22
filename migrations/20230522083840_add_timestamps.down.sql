-- DROP down migration script here

DROP TRIGGER IF EXISTS update_users_modified_at ON users;
DROP TRIGGER IF EXISTS update_user_profiles_modified_at  ON user_profiles;
DROP TRIGGER IF EXISTS update_users_osu_data_modified_at ON users_osu_data;
DROP TRIGGER IF EXISTS update_influences_modified_at ON influences;
DROP TRIGGER IF EXISTS update_user_osu_maps_modified_at ON user_osu_maps;

DROP FUNCTION IF EXISTS update_modification_date();

ALTER TABLE users DROP COLUMN IF EXISTS created_at;
ALTER TABLE users DROP COLUMN IF EXISTS modified_at;

ALTER TABLE user_profiles DROP COLUMN IF EXISTS created_at;
ALTER TABLE user_profiles DROP COLUMN IF EXISTS modified_at;

ALTER TABLE users_osu_data DROP COLUMN IF EXISTS created_at;

ALTER TABLE influences DROP COLUMN IF EXISTS created_at;
ALTER TABLE influences DROP COLUMN IF EXISTS modified_at;

ALTER TABLE user_osu_maps DROP COLUMN IF EXISTS created_at;
ALTER TABLE user_osu_maps DROP COLUMN IF EXISTS modified_at;
