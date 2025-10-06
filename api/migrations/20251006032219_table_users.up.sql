CREATE TABLE
    IF NOT EXISTS users (
        id UUID PRIMARY KEY,
        name VARCHAR(120) NOT NULL,
        phone VARCHAR(20) NOT NULL,
        email VARCHAR(255) NOT NULL,
        username VARCHAR(50) NOT NULL,
        hash VARCHAR(255) NOT NULL,
        is_admin BOOLEAN NOT NULL DEFAULT FALSE,
        is_verified BOOLEAN NOT NULL DEFAULT FALSE,
        created_at BIGINT NOT NULL,
        updated_at BIGINT NOT NULL,
        deleted_at BIGINT,
        UNIQUE (email),
        UNIQUE (username)
    );

CREATE TABLE
    IF NOT EXISTS user_tokens (
        user_id UUID NOT NULL,
        confirm_token VARCHAR(255),
        reset_token VARCHAR(255),
        token_expires_at BIGINT,
        PRIMARY KEY (user_id),
        FOREIGN KEY (user_id) REFERENCES users (id)
    );