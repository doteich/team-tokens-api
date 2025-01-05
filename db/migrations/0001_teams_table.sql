CREATE TABLE teams
(
    id bigserial NOT NULL,
    owner "char" NOT NULL,
    name "char" NOT NULL,
    created_at date NOT NULL,
    PRIMARY KEY (id)
);
