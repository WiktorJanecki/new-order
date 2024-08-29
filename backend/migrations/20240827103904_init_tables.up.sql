-- Add up migration script here

CREATE TYPE Privileges AS ENUM ('Basic', 'Full');

CREATE TABLE users(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    privileges Privileges NOT NULL
);

CREATE TABLE orders(
    id SERIAL PRIMARY KEY NOT NULL,
    creator_id INT NOT NULL,
    time_created TIMESTAMP NOT NULL,
    receiver VARCHAR NOT NULL,
    additional_info VARCHAR, 
    deleted BOOLEAN NOT NULL,
    paid BOOLEAN NOT NULL
);

CREATE TABLE items(
    id SERIAL PRIMARY KEY NOT NULL,
    order_id INT NOT NULL,
    creator_id INT NOT NULL,
    time_created TIMESTAMP NOT NULL,

    quantity VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    value INT NOT NULL,
    additional_info VARCHAR,
    deleted BOOLEAN NOT NULL
);
