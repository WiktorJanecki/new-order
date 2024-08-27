-- Add down migration script here
DELETE FROM users WHERE name="worker" OR name="boss";
