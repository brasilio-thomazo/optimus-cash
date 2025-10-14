-- Update user permissions
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

-- Get roles
CREATE
OR REPLACE FUNCTION get_roles (ids INT[]) RETURNS jsonb AS $$
BEGIN
    RETURN (
        SELECT JSON_AGG(t) FROM (SELECT "path", "method" FROM roles WHERE "id" = ANY(ids)) t
    );
END;
$$ LANGUAGE plpgsql;

-- Get permissions
CREATE
OR REPLACE FUNCTION get_permissions ("ids" INT[]) RETURNS JSONB AS $$
BEGIN
    RETURN (
        SELECT jsonb_agg(jsonb_build_object('id', id, 'roles', roles))
        FROM (
            SELECT g.id, JSONB_AGG(arr) AS roles
            FROM groups g, jsonb_array_elements(g.roles) AS arr
            WHERE g.id = ANY(ids)
            GROUP BY g.id
        )
    );
END;
$$ LANGUAGE plpgsql;

-- Triggers
-- Timestamp
CREATE TRIGGER update_timestamp BEFORE
UPDATE ON roles FOR EACH ROW
EXECUTE PROCEDURE update_timestamp ();

-- Permissions
CREATE TRIGGER update_user_permissions
AFTER
UPDATE OF roles ON "groups" FOR EACH ROW
EXECUTE PROCEDURE update_user_permissions ();