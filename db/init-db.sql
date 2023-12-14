CREATE TABLE IF NOT EXISTS users (
  id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
  name character varying NOT NULL,
  email character varying NOT NULL,
  password character varying NOT NULL,
  created_at timestamp(6) without time zone NOT NULL,
  updated_at timestamp(6) without time zone NOT NULL
);
CREATE UNIQUE INDEX index_users_on_id ON users(id uuid_ops);
CREATE UNIQUE INDEX index_users_on_email ON users(email text_ops);

INSERT INTO users (name, email, password, created_at, updated_at) VALUES
('Juan Perez', 'jperez@domain.tk','passw0rd', NOW() - interval '7 days', NOW()),
('Pedro Campos', 'pcampos@domain.tk','passw0rd', NOW() - interval '7 days', NOW());
