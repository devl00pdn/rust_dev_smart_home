#!/bin/sh
curl --request GET \
    --url http://localhost:8080/room \
    --header "Content-Type: application/json"