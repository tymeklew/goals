CREATE TABLE IF NOT EXISTS users (
   id uuid NOT NULL,
   email text NOT NULL UNIQUE,
   first_name text NOT NULL,
   last_name text NOT NULL,
   password text NOT NULL,
   PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS sessions (
   id 
);
