#!/bin/sh
ROOM_ID=$1
#DEVICE_NAME=$2
#DEVICE_TYPE=$3

curl --request DELETE \
    --url "http://localhost:8080/device/$ROOM_ID" \
    --header "Content-Type: application/json" \
