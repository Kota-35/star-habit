-- Add down migration script here
ALTER TABLE users ADD COLUMN firebase_uid TEXT UNIQUE;
