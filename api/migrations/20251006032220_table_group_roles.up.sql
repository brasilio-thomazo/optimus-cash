CREATE TABLE
    IF NOT EXISTS group_roles (
        group_id UUID NOT NULL,
        role_id UUID NOT NULL,
        PRIMARY KEY (group_id, role_id),
        FOREIGN KEY (group_id) REFERENCES groups (id),
        FOREIGN KEY (role_id) REFERENCES roles (id)
    );