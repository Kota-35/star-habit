-- Add up migration script here
CREATE TYPE auth_provider_id AS ENUM (
  'password',
  'phone',
  'google.com',
  'facebook.com'
);

CREATE TABLE auth_methods (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  provider_id auth_provider_id NOT NULL,
  provider_uid TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  UNIQUE (provider_id, provider_uid)
);

CREATE INDEX idx_auth_methods_user_id ON auth_methods(user_id);
CREATE INDEX idx_auth_methods_provider ON auth_methods(provider_id, provider_uid);