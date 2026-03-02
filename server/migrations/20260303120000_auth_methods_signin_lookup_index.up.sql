-- サインイン時の「provider_uid + provider_id で user を取得」クエリを高速化するため、
-- インデックスオンリースキャン可能な複合インデックスを追加し、統計を更新する。
CREATE INDEX IF NOT EXISTS idx_auth_methods_provider_include_user_id
  ON auth_methods (provider_id, provider_uid)
  INCLUDE (user_id);

ANALYZE auth_methods;
