#!/bin/bash
set -e

if [ ! -f "dist/desktop" ]; then
    echo "Error: dist/desktop not found. Build it first."
    exit 1
fi

echo "Measuring idle CPU usage..."

xvfb-run -a bash -c './dist/desktop' > /dev/null 2>&1 &
XVFB_PID=$!

echo "Letting process settle for 2 seconds..."
sleep 2

# The actual desktop process is a descendant of XVFB_PID
# Find the PID of the desktop executable
DESKTOP_PID=$(pgrep -x desktop | head -n 1)

if [ -z "$DESKTOP_PID" ]; then
    echo "Error: Could not find desktop process."
    kill $XVFB_PID
    exit 1
fi

echo "Found desktop process: $DESKTOP_PID"

echo "Sampling CPU for 5 seconds..."
top -b -n 5 -d 1 -p $DESKTOP_PID > top_output.txt

kill $XVFB_PID
wait $XVFB_PID 2>/dev/null || true
# Kill leftover desktop process if any
killall desktop 2>/dev/null || true

awk -v pid="$DESKTOP_PID" '$1 == pid {print $9}' top_output.txt > cpu_samples.txt

if [ -s cpu_samples.txt ]; then
    AVG_CPU=$(awk '{ sum += $1; n++ } END { if (n > 0) print sum / n; else print 0 }' cpu_samples.txt)
    echo "Average Idle CPU usage: $AVG_CPU%"
else
    echo "Could not collect CPU samples."
fi

rm -f top_output.txt cpu_samples.txt
