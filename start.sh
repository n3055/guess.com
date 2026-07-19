#!/bin/bash

# Set default port if not provided by Railway
export PORT=${PORT:-8080}

# Replace LISTEN_PORT placeholder in nginx configuration with the actual port
sed -i "s/LISTEN_PORT/${PORT}/g" /etc/nginx/sites-available/default

# Start the Rust web/WS app in the background, listening on 127.0.0.1
export HOST=127.0.0.1
export PORT=3000
./target/release/hello-world &

# Start nginx in the foreground
exec nginx -g "daemon off;"
