#!/bin/sh
set -e

npm install && npx tailwindcss -i static/css/styles.css -o static/css/main.css && zola build
