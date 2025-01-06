CREATE TABLE teams
(
    id bigserial NOT NULL,
    owner "text" NOT NULL,
    name "text" NOT NULL,
    created_at timestamptz NOT NULL,
    PRIMARY KEY (id)
);
