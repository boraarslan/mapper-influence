-- Add up migration script here

-- No need to add error modification date as it already 
-- has creation date and it's not going to be modified

ALTER TABLE users ADD created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE users ADD modified_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP;

ALTER TABLE user_profiles ADD created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE user_profiles ADD modified_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP;

ALTER TABLE users_osu_data ADD created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP;
--users_osu_data already has modified_at column

ALTER TABLE influences ADD created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE influences ADD modified_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP;

ALTER TABLE user_osu_maps ADD created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE user_osu_maps ADD modified_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP;

CREATE  FUNCTION update_modification_date()
RETURNS TRIGGER AS $$
BEGIN
    NEW.modified_at = (now() AT TIME ZONE 'utc');
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_modified_at BEFORE UPDATE ON users FOR EACH ROW EXECUTE PROCEDURE  update_modification_date();
CREATE TRIGGER update_user_profiles_modified_at BEFORE UPDATE ON user_profiles FOR EACH ROW EXECUTE PROCEDURE  update_modification_date();
CREATE TRIGGER update_users_osu_data_modified_at BEFORE UPDATE ON users_osu_data FOR EACH ROW EXECUTE PROCEDURE  update_modification_date();
CREATE TRIGGER update_influences_modified_at BEFORE UPDATE ON influences FOR EACH ROW EXECUTE PROCEDURE  update_modification_date();
CREATE TRIGGER update_user_osu_maps_modified_at BEFORE UPDATE ON user_osu_maps FOR EACH ROW EXECUTE PROCEDURE  update_modification_date();
