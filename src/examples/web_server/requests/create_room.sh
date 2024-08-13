#!/bin/sh
ROOM_NAME=$1

curl --request POST \
    --url http://localhost:8080/room \
    --header "Content-Type: application/json" \
    --data '{"name": "'"$ROOM_NAME"'", "devices": []}'