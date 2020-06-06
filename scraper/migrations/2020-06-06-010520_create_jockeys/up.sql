CREATE TABLE IF NOT EXISTS jockeys (
    id TEXT PRIMARY KEY,
    born TEXT,
    blood_type TEXT,
    height INT,
    weight INT,
    inserted_at TIMESTAMP DEFAULT NOW()
);