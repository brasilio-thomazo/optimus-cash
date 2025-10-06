CREATE TABLE
    IF NOT EXISTS roles (
        id UUID PRIMARY KEY,
        name VARCHAR(50) NOT NULL,
        route VARCHAR(255) NOT NULL,
        permissions JSONB NOT NULL DEFAULT '[]',
        created_at BIGINT NOT NULL,
        updated_at BIGINT NOT NULL,
        deleted_at BIGINT,
        UNIQUE (name, route)
    );