INSERT INTO
    roles ("name", endpoint, "method")
VALUES
    ('total access', '/', 'ANY'),
    ('groups access', '/groups', 'ANY'),
    ('users access', '/users', 'ANY'),
    ('show user', '/users', 'GET'),
    ('create user', '/users', 'POST'),
    ('update user', '/users', 'PUT'),
    ('patch user', '/users', 'PATCH'),
    ('delete user', '/users', 'DELETE'),
    ('show group', '/groups', 'GET'),
    ('create group', '/groups', 'POST'),
    ('update group', '/groups', 'PUT'),
    ('patch group', '/groups', 'PATCH'),
    ('delete group', '/groups', 'DELETE'),
    ('branch access', '/branches', 'ANY'),
    ('show branch', '/branches', 'GET'),
    ('edit branch', '/branches', 'PUT'),
    ('patch branch', '/branches', 'PATCH'),
    ('delete branch', '/branches', 'DELETE'),
    ('show branch accout', '/branch-accounts', 'GET'),
    ('edit branch account', '/branch-accounts', 'PUT'),
    ('patch branch account', '/branch-accounts', 'PATCH'),
    ('delete branch account', '/branch-accounts', 'DELETE') ON CONFLICT ("name", endpoint, "method")
DO NOTHING;