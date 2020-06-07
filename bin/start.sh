#!/usr/bin/env bash

# Load dotenv
export $(egrep -v '^#' .env | xargs)

# Start up postgres
docker-compose up -d postgres

until docker-compose exec postgres /usr/bin/pg_isready
do
  echo "Waiting for PostgreSQL to..."
  sleep 1
done


# Start up pgweb
docker-compose up -d pgweb