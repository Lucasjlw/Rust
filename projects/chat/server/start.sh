#!/bin/bash

# Start the Rust chat server in the background
/usr/local/bin/chat-server &

# Start Nginx in the foreground
nginx -g 'daemon off;'