#!/bin/bash
set -e

if [ ! -f "dist/desktop" ]; then
    echo "Error: dist/desktop not found. Build it first."
    exit 1
fi

echo "Measuring startup time..."

calculate_percentiles() {
    local sorted_array=($(printf '%s\n' "$@" | sort -n))
    local count=${#sorted_array[@]}

    local p50_idx=$(awk -v c=$count 'BEGIN { idx = int((50 / 100.0) * c + 0.5) - 1; if (idx < 0) idx = 0; print idx }')
    local p95_idx=$(awk -v c=$count 'BEGIN { idx = int((95 / 100.0) * c + 0.5) - 1; if (idx < 0) idx = 0; print idx }')

    echo "p50: ${sorted_array[$p50_idx]} ms, p95: ${sorted_array[$p95_idx]} ms"
}

run_once() {
    local START=$(date +%s%3N)

    # Run the app under xvfb-run
    xvfb-run -e /dev/stderr -a bash -c '
        ./dist/desktop > /dev/null 2>&1 &
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
    local END=$(date +%s%3N)

    echo $((END - START))
}

cold_runs=()
echo "Running 5 cold samples..."
for i in {1..5}; do
    sync
    echo 3 > /proc/sys/vm/drop_caches 2>/dev/null || echo "Warning: Could not drop caches (requires root), cold measurement may be inaccurate."
    cold_runs+=($(run_once))
done

warm_runs=()
echo "Running 20 warm samples..."
for i in {1..20}; do
    warm_runs+=($(run_once))
done

echo "Cold launch percentiles:"
calculate_percentiles "${cold_runs[@]}"

echo "Warm launch percentiles:"
calculate_percentiles "${warm_runs[@]}"
