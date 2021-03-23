-- This file should undo anything in `up.sql`
DROP TABLE roles;
ALTER TABLE users
DROP role_id;