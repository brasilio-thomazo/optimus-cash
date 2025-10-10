CREATE TABLE IF NOT EXISTS
    GROUPS (
        id UUID PRIMARY KEY,
        "name" VARCHAR(50) NOT NULL,
        roles JSONB NOT NULL DEFAULT '[]',
        description VARCHAR(255),
        created_at BIGINT NOT NULL,
        updated_at BIGINT NOT NULL,
        deleted_at BIGINT,
        UNIQUE ("name")
    );