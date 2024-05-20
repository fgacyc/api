-- migrate:up
CREATE TABLE connect_group_category (
    id TEXT,
    name TEXT NOT NULL,
    PRIMARY KEY(id),
    UNIQUE(name)
);
COMMENT ON COLUMN connect_group_category.id IS 'Unique identifier of a connect group category (e.g., connect_group_category_01H7JNPD7J67AA5AD87Q4SZDF9)';
COMMENT ON COLUMN connect_group_category.name IS 'Name for a connect group category.';

ALTER TABLE connect_group ADD COLUMN category_id TEXT NOT NULL REFERENCES connect_group_category(id);
ALTER TABLE connect_group ADD COLUMN active BOOLEAN NOT NULL DEFAULT true;
ALTER TABLE connect_group ADD COLUMN closed_at TIMESTAMPTZ;

-- migrate:down
ALTER TABLE connect_group DROP COLUMN closed_at;
ALTER TABLE connect_group DROP COLUMN active;
ALTER TABLE connect_group DROP COLUMN category_id;

DROP TABLE connect_group_category;