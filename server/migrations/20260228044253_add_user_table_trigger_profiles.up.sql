CREATE TRIGGER trg_profiles_set_updated_at
BEFORE UPDATE ON profiles
FOR EACH ROW
EXECUTE PROCEDURE set_updated_at();
