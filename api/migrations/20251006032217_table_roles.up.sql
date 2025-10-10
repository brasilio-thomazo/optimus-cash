CREATE TYPE role_method AS ENUM('GET', 'POST', 'PUT', 'PATCH', 'DELETE', 'ANY');

CREATE TABLE IF NOT EXISTS
    roles (
        id SERIAL PRIMARY KEY,
        "name" VARCHAR(50) NOT NULL,
        endpoint VARCHAR(50) NOT NULL,
        "method" role_method NOT NULL,
        created_at BIGINT NOT NULL DEFAULT EXTRACT(
            EPOCH
            FROM
                NOW()
        ),
        updated_at BIGINT NOT NULL DEFAULT EXTRACT(
            EPOCH
            FROM
                NOW()
        ),
        deleted_at BIGINT,
        UNIQUE ("name"),
        UNIQUE (endpoint, "method")
    );

CREATE
OR REPLACE FUNCTION update_timestamp () RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = EXTRACT(EPOCH FROM NOW());
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_timestamp BEFORE
UPDATE ON roles FOR EACH ROW
EXECUTE PROCEDURE update_timestamp ();