#!/bin/sh
ROOM_ID=$1
DEVICE_NAME=$2

curl --request DELETE \
    --url "http://localhost:8080/room/$ROOM_ID/device/$DEVICE_NAME" \
    --header "Content-Type: application/json"