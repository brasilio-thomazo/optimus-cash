CREATE TYPE role_method AS ENUM ('GET', 'POST', 'PUT', 'DELETE', 'ANY');

CREATE TABLE
    IF NOT EXISTS roles (
        id UUID PRIMARY KEY,
        name VARCHAR(50) NOT NULL,
        endpoint VARCHAR(50) NOT NULL,
        method role_method NOT NULL,
        created_at BIGINT NOT NULL,
        updated_at BIGINT NOT NULL,
        deleted_at BIGINT,
        UNIQUE (name, endpoint, method)
    );