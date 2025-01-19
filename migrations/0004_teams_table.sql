CREATE TABLE teams (
    id bigserial NOT NULL,
    owner_name "text" NOT NULL,
    owner_id bigserial NOT NULL,
    name "text" NOT NULL,
    created_at timestamptz NOT NULL,
    PRIMARY KEY (id),
    ADD CONSTRAINT fk_owner_id FOREIGN KEY (owner_id) REFERENCES public.users (id) MATCH SIMPLE ON UPDATE NO ACTION ON DELETE NO ACTION NOT VALID;
);