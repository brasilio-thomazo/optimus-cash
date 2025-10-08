INSERT INTO
    users (
        id,
        name,
        phone,
        email,
        username,
        hash,
        is_admin,
        is_verified,
        created_at,
        updated_at
    )
VALUES
    (
        gen_random_uuid (),
        'admin',
        '',
        'postmaster@localhost',
        'admin',
        '$argon2id$v=19$m=32768,t=1,p=1$JRa6pp7nIa5ny3QAZenJGg$e/b0fkoNSCVIl/yNF9Qq9Ym9e0lrCu2SW0LzyhR3lLY',
        true,
        true,
        extract(
            epoch
            from
                now ()
        ),
        extract(
            epoch
            from
                now ()
        )
    );