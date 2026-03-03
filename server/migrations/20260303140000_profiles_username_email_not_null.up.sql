-- profiles.username / profiles.email を NOT NULL にし、サインアップで必須であることを DB で保証する。
-- 既存で NULL の行がある場合は、空文字に更新してから制約を付与する。
UPDATE profiles SET username = '' WHERE username IS NULL;
UPDATE profiles SET email = '' WHERE email IS NULL;

ALTER TABLE profiles
  ALTER COLUMN username SET NOT NULL,
  ALTER COLUMN email SET NOT NULL;
