#!/bin/bash

if [ -z "$1" ] || [ "$1" = "--help" ]; then
    echo "Usage: $0 <executable> [args...]"
    exit 0
fi

if ! command -v xvfb-run >/dev/null 2>&1; then
    echo "Error: xvfb-run is required."
    exit 1
fi

if ! command -v xdotool >/dev/null 2>&1; then
    echo "Error: xdotool is required."
    exit 1
fi

xvfb-run -a bash -c '
    start_time=$(date +%s%3N)
    "$@" &
    APP_PID=$!

    window_found=false
    for i in {1..100}; do
        if xdotool search --onlyvisible ".*" >/dev/null 2>&1; then
            window_found=true
            break
        fi
        sleep 0.1
    done

    end_time=$(date +%s%3N)
    kill $APP_PID 2>/dev/null || true
    wait $APP_PID 2>/dev/null || true

    if [ "$window_found" = true ]; then
        echo "Startup time: $((end_time - start_time)) ms"
    else
        echo "Error: Window did not appear within 10 seconds."
        exit 1
    fi
' _ "$@"
