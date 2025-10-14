CREATE TABLE IF NOT EXISTS
    GROUPS (
        id SERIAL PRIMARY KEY,
        "name" VARCHAR(50) NOT NULL,
        roles JSONB NOT NULL DEFAULT '[]',
        description VARCHAR(255),
        created_at BIGINT NOT NULL DEFAULT get_timestamp (),
        updated_at BIGINT NOT NULL DEFAULT get_timestamp (),
        deleted_at BIGINT,
        UNIQUE ("name")
    );