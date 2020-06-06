CREATE TABLE IF NOT EXISTS jockeys (
    id TEXT PRIMARY KEY,
    name TEXT,
    born TEXT,
    birthday TEXT,
    blood_type TEXT,
    height INT,
    weight INT,
    inserted_at TIMESTAMP DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS jockeys_id_idx ON jockeys (id);