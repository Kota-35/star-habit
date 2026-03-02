-- Add down migration script here
DROP TABLE IF EXISTS auth_methods;
DROP TYPE IF EXISTS auth_provider_id;
