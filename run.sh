#!/usr/bin/env bash

TOKEN=`cat ~/.cipherstash/default/auth-token.json | jq '.accessToken' | sed s/\"//g`

WORKER_BASE_URL=${WORKER_BASE_URL:-"http://localhost:8787"}

# Insert a User
insert_user() {
  curl -H "Content-Type: application/json; charset=utf-8" \
    -H "Authorization: Bearer ${TOKEN}"\
    -XPOST -d "$1" -s \
    $WORKER_BASE_URL
  echo
  echo
}

# Query Users using 
query_users() {
  echo "Sending query $1" 

  curl -H "Content-Type: application/json; charset=utf-8" \
    -H "Authorization: Bearer ${TOKEN}" \
    -XPOST -d "$1" -s \
    $WORKER_BASE_URL/query | jq '.'

  echo
  echo
}

# Get a User by their UUID
get_user() {
  curl -H "Authorization: Bearer ${TOKEN}" -s \
    "$WORKER_BASE_URL/$1"
  echo
  echo
}

echo "Insert a certain user"
insert_user '{ "id": "aae81d62-b80b-4b45-a622-4447510a3cba", "name": "Bob", "email": "bob@bob.co", "dob": "1998-01-31T14:00:00.000Z" }'

echo "Get a certain user by id"
get_user "aae81d62-b80b-4b45-a622-4447510a3cba"

echo "Find users where their email fuzzy-matches gmail"
query_users '{ "email": { "op": "match", "value": "gmail" }, "limit": 5 }'

echo "Find the first 5 users born before 1980"
query_users '{ "dob": { "op": "lt", "value": "1980-01-01T00:00:00.000Z" }, "limit": 5 }'

echo "Find the first 5 users born after 1980"
query_users '{ "dob": { "op": "gt", "value": "1980-01-01T00:00:00.000Z" }, "limit": 5 }'

echo "Find the first 5 users born after 1960"
query_users '{ "dob": { "op": "gt", "value": "1960-01-01T00:00:00.000Z" }, "limit": 5 }'

echo "Find users that match a certain name"
query_users '{ "name": { "op": "eq", "value": "Casey Greenfelder" } }'

echo "Find users with a hotmail email born after 1960"
query_users '{ "email": { "op": "match", "value": "hotmail" }, "dob": { "op": "gt", "value": "1960-01-01T00:00:00.000Z" }, "limit": 5 }'
