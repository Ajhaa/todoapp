DROP TRIGGER stamp_todos ON todos;
DROP TRIGGER stamp_users ON users;

DROP TABLE users;
DROP TABLE todos;
DROP TABLE base;

DROP FUNCTION create_user(text, text);
DROP FUNCTION get_user(text, text);

DROP FUNCTION update_timestamp();

DROP EXTENSION "pgcrypto";

