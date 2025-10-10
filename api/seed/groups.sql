INSERT INTO
    "groups" (id, "name", roles, created_at, updated_at)
VALUES
    (
        gen_random_uuid (),
        'admin',
        (
            SELECT
                JSON_AGG(t)
            FROM
                (
                    SELECT
                        endpoint,
                        "method"
                    FROM
                        roles
                    WHERE
                        "name" = 'total access'
                ) t
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
    ),
    (
        gen_random_uuid (),
        'user',
        (
            SELECT
                JSON_AGG(t)
            FROM
                (
                    SELECT
                        endpoint,
                        "method"
                    FROM
                        roles
                    WHERE
                        "name" IN ('show user', 'show group', 'show branch', 'show branch accout')
                ) t
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
    ) ON CONFLICT ("name")
DO NOTHING;