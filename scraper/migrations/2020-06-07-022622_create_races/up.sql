CREATE TABLE IF NOT EXISTS races (
    inserted_at TIMESTAMP DEFAULT NOW(),
    id TEXT PRIMARY KEY,
    name TEXT,
    race_date TEXT,
    start_time TEXT,
    race_type TEXT,
    meter INT,
    weather TEXT,
    condition TEXT,
    qualifications TEXT,
    other_detail TEXT
);
CREATE INDEX IF NOT EXISTS races_id_idx ON races (id);