
CREATE TABLE electricity_prices (
    id SERIAL PRIMARY KEY,
    datetime TIMESTAMPTZ NOT NULL, 
    value NUMERIC(10, 4) NOT NULL,
    geo_zone VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(datetime, geo_zone) 
);