CREATE TABLE report_history (
    id SERIAL PRIMARY KEY,
    total INTEGER NOT NULL,
    progress_percent DOUBLE PRECISION NOT NULL,
    average_gap DOUBLE PRECISION NOT NULL,
    analyzed_at TIMESTAMPTZ NOT NULL DEFAULT now()
);   