INSERT INTO
    users (id, "name", phone, email, username, "hash", is_admin, is_verified, permissions, created_at, updated_at)
VALUES
    (
        gen_random_uuid (),
        'admin',
        '',
        'postmaster@localhost',
        'admin',
        '$argon2id$v=19$m=32768,t=1,p=1$JRa6pp7nIa5ny3QAZenJGg$e/b0fkoNSCVIl/yNF9Qq9Ym9e0lrCu2SW0LzyhR3lLY',
        TRUE,
        TRUE,
        (
            SELECT
                JSONB_AGG(JSON_BUILD_OBJECT('id', id, 'roles', a))
            FROM
                (
                    SELECT
                        g.id,
                        JSONB_AGG(b) AS a
                    FROM
                        GROUPS g,
                        JSONB_ARRAY_ELEMENTS(roles) b
                    WHERE
                        g.name IN ('admin')
                    GROUP BY
                        g.id
                )
        ),
        EXTRACT(
            epoch
            FROM
                NOW()
        ),
        EXTRACT(
            epoch
            FROM
                NOW()
        )
    );