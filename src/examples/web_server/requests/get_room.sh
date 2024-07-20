#!/bin/sh
ROOM_ID=$1

curl --request GET \
    --url "http://localhost:8080/room/ROOM_ID" \
    --header "Content-Type: application/json"