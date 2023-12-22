CREATE TABLE IF NOT EXISTS users (
   id uuid NOT NULL,
   email text NOT NULL UNIQUE,
   first_name text NOT NULL,
   last_name text NOT NULL,
   password text NOT NULL,
   PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS sessions (
   id uuid NOT NULL,
   user_id uuid NOT NULL,
   PRIMARY KEY (id),
   FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS goals (
   id uuid NOT NULL,
   user_id uuid NOT NULL,
   title text NOT NULL,
   description text NOT NULL,
   PRIMARY KEY (id),
   FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS resets (
   id uuid NOT NULL,
   user_id uuid NOT NULL,
   token text NOT NULL
); 
