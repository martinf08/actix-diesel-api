-- This file should undo anything in `up.sql`
SET FOREIGN_KEY_CHECKS=0;
DROP TABLE IF EXISTS roles;
ALTER TABLE users
    DROP FOREIGN KEY users_ibfk_1,
    DROP role_id;
SET FOREIGN_KEY_CHECKS=1;