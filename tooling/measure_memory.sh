#!/bin/bash
set -e

if [ ! -f "dist/desktop" ]; then
    echo "Error: dist/desktop not found."
    exit 1
fi

# Start the desktop process under xvfb-run to provide a display
xvfb-run -a dist/desktop &
PID=$!

# Wait for process to settle
sleep 2

if ! kill -0 $PID 2>/dev/null; then
    echo "Error: Process $PID crashed."
    exit 1
fi

# Use pgrep and smaps to get the total private memory of the process and its children
PIDS=$(pgrep -P $PID)
ALL_PIDS="$PID $PIDS"

TOTAL_PRIVATE=0
for p in $ALL_PIDS; do
    if [ -f "/proc/$p/smaps" ]; then
        PRIVATE=$(awk '/^Private_/ {sum += $2} END {if (sum == "") print 0; else print sum}' "/proc/$p/smaps" 2>/dev/null || echo 0)
        TOTAL_PRIVATE=$((TOTAL_PRIVATE + PRIVATE))
    else
        RSS=$(ps -o rss= -p $p 2>/dev/null || echo 0)
        TOTAL_PRIVATE=$((TOTAL_PRIVATE + RSS))
    fi

done

# Convert KB to MB
TOTAL_MB=$(awk "BEGIN {printf \"%.2f\",  $TOTAL_PRIVATE / 1024}")
echo "Memory: $TOTAL_MB MB"

# Clean up
kill $PID
wait $PID 2>/dev/null || true
