CREATE TABLE
    IF NOT EXISTS user_groups (
        user_id UUID NOT NULL,
        group_id UUID NOT NULL,
        PRIMARY KEY (user_id, group_id),
        FOREIGN KEY (user_id) REFERENCES users (id),
        FOREIGN KEY (group_id) REFERENCES groups (id)
    );