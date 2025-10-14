-- Generate timestamp
CREATE
OR REPLACE FUNCTION get_timestamp () RETURNS BIGINT AS $$
BEGIN
    RETURN EXTRACT(EPOCH FROM NOW());
END;
$$ LANGUAGE plpgsql;

-- Update timestamp
CREATE
OR REPLACE FUNCTION update_timestamp () RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = get_timestamp();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;