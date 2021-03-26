-- This file should undo anything in `up.sql`
ALTER TABLE users
    DROP FOREIGN KEY users_ibfk_1,
    DROP role_id;
DROP TABLE IF EXISTS roles;
