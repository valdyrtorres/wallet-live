-- Add up migration script here
CREATE TABLE purchase_history (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    asset_id BIGINT NOT NULL REFERENCES assets(id),
    bought_at TIMESTAMPTZ NOT NULL,
    bought_for DOUBLE PRECISION NOT NULL,
    quantity_bought DOUBLE PRECISION NOT NULL,
    value_delta DOUBLE PRECISION NOT NULL
);