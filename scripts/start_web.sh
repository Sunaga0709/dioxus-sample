#!/bin/bash

cd front

dx serve --bin web --hot-reload & \
npx tailwind -i ./input.css -o public/tailwind.css --watch & \
wait

cd ../
