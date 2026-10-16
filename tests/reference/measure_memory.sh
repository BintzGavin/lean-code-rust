#!/bin/bash

if [ -z "$1" ] || [ "$1" = "--help" ]; then
    echo "Usage: $0 <PID>"
    exit 0
fi

PID=$1

if ! kill -0 $PID 2>/dev/null; then
    echo "Error: Process $PID not found."
    exit 1
fi

get_process_tree() {
    local pid=$1
    echo $pid
    for child in $(pgrep -P $pid 2>/dev/null); do
        get_process_tree $child
    done
}

pids=$(get_process_tree $PID)

total_memory=0
for p in $pids; do
    if [ -f "/proc/$p/smaps" ]; then
        mem=$(awk '/^Private_Dirty:/ {sum += $2} END {print sum}' "/proc/$p/smaps")
        if [ ! -z "$mem" ]; then
            total_memory=$((total_memory + mem))
        fi
    fi
done

echo "Total process-tree memory (Private_Dirty): ${total_memory} kB"
