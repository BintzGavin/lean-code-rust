#!/bin/bash
set -e

if [ ! -f "dist/desktop" ]; then
    echo "Error: dist/desktop not found. Build it first."
    exit 1
fi

echo "Measuring idle network activity..."

xvfb-run -a strace -f -e trace=network -o strace_network.out ./dist/desktop > /dev/null 2>&1 &
PID=$!

sleep 5

kill $PID
wait $PID 2>/dev/null || true

NETWORK_REQUESTS=$(grep -E 'socket\(AF_INET|connect\(.*AF_INET|sendto\(.*AF_INET|sendmsg\(.*AF_INET' strace_network.out | wc -l || true)

echo "Network requests: $NETWORK_REQUESTS"

if [ "$NETWORK_REQUESTS" -ne 0 ]; then
    echo "Error: Unexpected network activity detected."
    grep -E 'socket\(AF_INET|connect\(.*AF_INET|sendto\(.*AF_INET|sendmsg\(.*AF_INET' strace_network.out
    rm -f strace_network.out
    exit 1
fi

rm -f strace_network.out
