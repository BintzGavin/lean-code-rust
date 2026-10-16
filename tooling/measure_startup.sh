#!/bin/bash
set -e

if [ ! -f "dist/desktop" ]; then
    echo "Error: dist/desktop not found. Build it first."
    exit 1
fi

echo "Measuring startup time..."
START=$(date +%s%3N)

# Run the app under xvfb-run
xvfb-run -a bash -c '
    ./dist/desktop &
    PID=$!

    # Wait until the window is created
    while true; do
        if xdotool search --onlyvisible --name "Lean Editor" >/dev/null 2>&1; then
            break
        fi
        # if the process died, exit loop
        if ! kill -0 $PID 2>/dev/null; then
            break
        fi
        sleep 0.05
    done
    kill $PID
    wait $PID 2>/dev/null || true
'
END=$(date +%s%3N)

STARTUP_MS=$((END - START))

echo "Startup time: $STARTUP_MS ms"
