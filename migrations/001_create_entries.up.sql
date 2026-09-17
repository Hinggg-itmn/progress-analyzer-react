CREATE TABLE entries (
    id SERIAL PRIMARY KEY,
    date DATE NOT NULL,
    name TEXT NOT NULL,
    category TEXT NOT NULL,
    difficulty TEXT NOT NULL CHECK (difficulty IN ('Easy', 'Medium', 'Hard')),
    approach TEXT NOT NULL,
    complexity TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);   