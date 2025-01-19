CREATE TABLE users (
    id bigserial NOT NULL,
    name "text" NOT NULL,
    password "text" NOT NULL,
    email "text" NOT NULL,
    created_at timestamptz NOT NULL,
    PRIMARY KEY (id)
);