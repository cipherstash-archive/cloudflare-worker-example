#!/usr/bin/env bash

set -eu

TOKEN=`cat ~/.cipherstash/default/auth-token.json | jq '.accessToken' | sed s/\"//g`

WORKER_BASE_URL=${WORKER_BASE_URL:-"http://localhost:8787"}

# Insert a User
insert_user() {
  curl -H "Content-Type: application/json; charset=utf-8" \
    -H "Authorization: Bearer ${TOKEN}" \
    -XPOST --data "$1" -s \
    $WORKER_BASE_URL

  echo
}

CURRENT=0;
COUNT=$(jq '. | length' example-users.json)

jq -c '.[]' example-users.json | while read i; do
    CURRENT=$((CURRENT+1));
    echo Creating user $CURRENT/$COUNT
    insert_user "$i"
done
