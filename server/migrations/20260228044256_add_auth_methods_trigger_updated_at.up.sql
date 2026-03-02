CREATE TRIGGER trg_auth_methods_set_updated_at
  BEFORE UPDATE ON auth_methods
  FOR EACH ROW
  EXECUTE PROCEDURE set_updated_at();
