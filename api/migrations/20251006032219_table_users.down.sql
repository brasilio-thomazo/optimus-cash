DROP TRIGGER IF EXISTS update_user_permissions ON "groups";

DROP FUNCTION IF EXISTS update_user_permissions ();

DROP INDEX IF EXISTS idx_users_permissions;

DROP TABLE IF EXISTS user_tokens;

DROP TABLE IF EXISTS users;