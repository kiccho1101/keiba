SELECT 'CREATE DATABASE keiba'
WHERE NOT EXISTS (
    SELECT
FROM pg_database
WHERE datname = 'keiba')
\gexec