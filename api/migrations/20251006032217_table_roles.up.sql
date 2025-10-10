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