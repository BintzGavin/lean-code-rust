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
    strace -e poll,epoll_wait,epoll_pwait -f -o strace_out.log ./dist/desktop &
    PID=$!

    # Wait until the application reaches the event loop (indicated by epoll_wait/poll)
    while true; do
        if grep -qE "epoll_wait|poll" strace_out.log 2>/dev/null; then
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
rm -f strace_out.log
