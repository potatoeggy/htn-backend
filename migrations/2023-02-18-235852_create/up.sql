-- Your SQL goes here

create table skills (
  id integer not null primary key,
  user_id integer not null references users (id),
  skill varchar(255) not null,
  rating int not null,
  created_at datetime not null default current_timestamp,
  updated_at datetime not null default current_timestamp,
  unique (user_id, skill) on conflict replace
);

create table users (
  id integer not null primary key,
  name varchar(255) not null,
  company varchar(255) not null,
  email varchar(255) not null,
  phone varchar(255) not null,
  created_at datetime not null default current_timestamp,
  updated_at datetime not null default current_timestamp
);