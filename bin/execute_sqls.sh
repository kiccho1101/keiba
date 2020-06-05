#!/bin/sh

# Load dotenv
export $(egrep -v '^#' .env | xargs)

# start up postgres
docker-compose up -d postgres
until docker-compose exec postgres /usr/bin/pg_isready
do
  echo "waiting for postgresql to..."
  sleep 1
done

# execute sql
for file in $(find postgres/init -type f)
do
    docker-compose exec postgres psql -U $POSTGRES_USER -d keiba -f ${file//postgres\/init/docker-entrypoint-initdb.d}
done