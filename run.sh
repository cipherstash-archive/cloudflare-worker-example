#!/bin/bash
token=`cat ~/.cipherstash/default/auth-token.json | jq '.accessToken' | sed s/\"//g`

curl -v -H "Authorization: Bearer ${token}" -XPOST http://localhost:8787

curl -v -H "Authorization: Bearer ${token}" -XGET http://localhost:8787/a3626964-d840-5075-bd35-b352544102b2
