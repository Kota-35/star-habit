CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  new.updated_at := now();
  RETURN new;
END;
$$ LANGUAGE plpgsql;
