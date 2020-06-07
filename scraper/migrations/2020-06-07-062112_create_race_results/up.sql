CREATE TABLE IF NOT EXISTS race_results (
    id TEXT,
    rank INT,
    frame INT,
    number INT,
    horse_id INT,
    horse_name TEXT,
    horse_age INT,
    handicap DOUBLE PRECISION,
    jockey_id TEXT,
    jockey_name TEXT,
    time TEXT,
    rank_diff TEXT,
    time_index TEXT,
    pass_rank TEXT,
    three_furlongs DOUBLE PRECISION,
    ozz DOUBLE PRECISION,
    popularity INT,
    horse_weight TEXT,
    training_time TEXT,
    house_comment TEXT,
    other TEXT,
    trainer_id TEXT,
    trainer_name TEXT,
    owner_id TEXT,
    owner_name TEXT,
    prize DOUBLE PRECISION,
    inserted_at TIMESTAMP DEFAULT NOW(),
    PRIMARY KEY (id, rank)
);
CREATE INDEX IF NOT EXISTS race_results_id_rank_idx ON race_results (id, rank);