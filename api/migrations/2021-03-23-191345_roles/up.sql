-- Your SQL goes here
CREATE TABLE roles
(
    id   INTEGER AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255)
);

ALTER TABLE users
    ADD COLUMN role_id INT,
    ADD CONSTRAINT FOREIGN KEY (role_id) REFERENCES roles(id)