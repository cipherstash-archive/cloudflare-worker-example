#!/bin/bash
token=`cat ~/.cipherstash/default/auth-token.json | jq '.accessToken' | sed s/\"//g`

curl -v -H "Authorization: Bearer ${token}" -XPOST http://localhost:8787
