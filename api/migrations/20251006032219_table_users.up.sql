CREATE TABLE IF NOT EXISTS
    users (
        id UUID PRIMARY KEY,
        "name" VARCHAR(120) NOT NULL,
        phone VARCHAR(20) NOT NULL,
        email VARCHAR(255) NOT NULL,
        username VARCHAR(50) NOT NULL,
        "hash" VARCHAR(255) NOT NULL,
        is_admin BOOLEAN NOT NULL DEFAULT FALSE,
        is_verified BOOLEAN NOT NULL DEFAULT FALSE,
        permissions JSONB NOT NULL DEFAULT '[]',
        created_at BIGINT NOT NULL,
        updated_at BIGINT NOT NULL,
        deleted_at BIGINT,
        UNIQUE (email),
        UNIQUE (username)
    );

CREATE TABLE IF NOT EXISTS
    user_tokens (
        user_id UUID NOT NULL,
        confirm_token VARCHAR(255),
        reset_token VARCHAR(255),
        token_expires_at BIGINT,
        PRIMARY KEY (user_id),
        FOREIGN KEY (user_id) REFERENCES users (id)
    );

CREATE INDEX idx_users_permissions ON users USING gin (permissions jsonb_path_ops);

CREATE
OR REPLACE FUNCTION update_user_permissions () RETURNS TRIGGER AS $$
DECLARE group_id TEXT := NEW.id::text;
DECLARE roles JSONB := NEW.roles;
BEGIN
    UPDATE users
    SET permissions = (
        SELECT jsonb_agg(
            CASE WHEN obj ->> 'id' = group_id THEN jsonb_set(obj, '{roles}', roles) ELSE obj END
        )
        FROM jsonb_array_elements(permissions) obj
    )
    WHERE permissions @> jsonb_build_array(jsonb_build_object('id', group_id));
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_user_permissions
AFTER
UPDATE OF roles ON "groups" FOR EACH ROW
EXECUTE PROCEDURE update_user_permissions ();