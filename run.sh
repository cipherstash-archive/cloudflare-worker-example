#!/usr/bin/env bash

TOKEN=`cat ~/.cipherstash/default/auth-token.json | jq '.accessToken' | sed s/\"//g`

WORKER_BASE_URL=${WORKER_BASE_URL:-"http://localhost:8787"}

# Insert a User
insert_user() {
  echo "Inserting user $(echo "$1" | jq '.name')"

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

  echo "Found $(
  curl -H "Content-Type: application/json; charset=utf-8" \
    -H "Authorization: Bearer ${TOKEN}" \
    -XPOST -d "$1" -s \
    $WORKER_BASE_URL/query | jq '.users | length') users"

  echo
  echo
}

# Get a User by their UUID
get_user() {
  echo "Getting user '$1'"

  curl -H "Authorization: Bearer ${TOKEN}" -s \
    "$WORKER_BASE_URL/$1"
  echo
  echo
}

insert_user '{ "id": "aae81d62-b80b-4b45-a622-4447510a3cba", "name": "Bob", "email": "bob@bob.co", "dob": "1998-01-31T14:00:00.000Z" }'

get_user "aae81d62-b80b-4b45-a622-4447510a3cba"

query_users '{ "email": { "op": "match", "value": "gmail" } }'
