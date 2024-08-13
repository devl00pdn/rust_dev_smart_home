#!/bin/sh
ROOM_ID=$1
DEVICE_NAME=$2
DEVICE_TYPE=$3

curl --request POST \
    --url "http://localhost:8080/room/$ROOM_ID/device" \
    --header "Content-Type: application/json" \
    --data '{"name": "'"$DEVICE_NAME"'", "description": "'"Description of $DEVICE_NAME"'", "device_type": "'"$DEVICE_TYPE"'"}'