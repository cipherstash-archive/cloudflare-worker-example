#!/bin/bash
token=`cat ~/.cipherstash/default/auth-token.json | jq '.accessToken' | sed s/\"//g`

curl -v -H "Content-Type: application/json; charset=utf-8"\
  -H "Authorization: Bearer ${token}"\
  -XPOST -d'{"id": "AAE81D62-B80B-4B45-A622-4447510A3CBA", "title":"The Matrix","runningTime":200}'\
  http://localhost:8787

curl -v -H "Authorization: Bearer ${token}" -XGET http://localhost:8787/aae81d62-b80b-4b45-a622-4447510a3cba
