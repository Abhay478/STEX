-- Your SQL goes here
-- create if not exists extension pgcrypto;
create table accounts (
    id INTEGER primary key,
    username VARCHAR(255),
    password_hash VARCHAR(127)
);

-- insert into accounts(id, username, password_hash) select id, display_name,  from users;
insert into accounts (id, username) select id, display_name from users;

--update accounts set password_hash = crypt(id || username, gen_salt('bf', 8));