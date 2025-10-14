CREATE TABLE IF NOT EXISTS
    roles (
        id SERIAL PRIMARY KEY,
        "path" VARCHAR(50) NOT NULL,
        "method" VARCHAR(6) NOT NULL CHECK ("method" IN ('ANY', 'GET', 'POST', 'PUT', 'PATCH', 'DELETE')),
        created_at BIGINT NOT NULL DEFAULT get_timestamp (),
        updated_at BIGINT NOT NULL DEFAULT get_timestamp (),
        deleted_at BIGINT,
        UNIQUE ("path", "method")
    );