#!/usr/bin/env bash

set -eu

TOKEN=`cat ~/.cipherstash/default/auth-token.json 2> /dev/null | jq '.accessToken' | sed s/\"//g`

if [ -z "$TOKEN" ]; then
  echo "Could not find CipherStash authentication token."
  echo 'Have you run `stash login --workspace <workspace>`?'
  exit 1
fi

WORKER_BASE_URL=${1:-}

if [ -z "$WORKER_BASE_URL" ]; then
  echo "Usage: ./bulk.sh <worker url>" >&2
  echo "Example: ./bulk.sh https://cipherstash-demo.foo.workers.dev" >&2
  exit 1
fi

# Insert a User
insert_user() {
  curl --fail-with-body -H "Content-Type: application/json; charset=utf-8" \
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
