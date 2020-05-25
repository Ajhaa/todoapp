CREATE EXTENSION "pgcrypto";

CREATE table base (
  created_at timestamp NOT NULL DEFAULT NOW(),
  updated_at timestamp NOT NULL DEFAULT NOW()
);

CREATE TABLE todos (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  content varchar NOT NULL,
  done boolean NOT NULL
) INHERITS (base);


CREATE TABLE users (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  username varchar(20) NOT NULL UNIQUE,
  password_hash text NOT NULL
) INHERITS (base);

-- hash and salt, blowfish with 10 salt rounds
CREATE FUNCTION create_user(text, text) RETURNS uuid AS
$$
  INSERT INTO users (username, password_hash) 
    VALUES ($1, crypt($2, gen_salt('bf', 10)))
    RETURNING id;
$$ LANGUAGE sql;

CREATE FUNCTION get_user(text, text) RETURNS TABLE(id uuid, username varchar(20)) AS
$$
  SELECT id, username FROM users 
    WHERE username = $1 AND password_hash = crypt($2, password_hash);
$$ LANGUAGE sql;

CREATE FUNCTION update_timestamp() RETURNS trigger AS 
$$
  BEGIN
    NEW.updated_at := NOW();
    RETURN NEW;
  END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER stamp_todos BEFORE UPDATE ON todos
  FOR EACH ROW
  EXECUTE PROCEDURE update_timestamp();

CREATE TRIGGER stamp_users BEFORE UPDATE on users
  FOR EACH ROW
  EXECUTE PROCEDURE update_timestamp();