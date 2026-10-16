#!/bin/bash
set -e

if [ ! -f "dist/desktop" ]; then
    echo "Error: dist/desktop not found."
    exit 1
fi

echo "Measuring Startup..."

# Use xvfb-run to manage Xvfb and authorization automatically
# Create a wrapper script to run our commands within the xvfb environment
cat << 'EOF' > /tmp/run_in_xvfb.sh
#!/bin/bash
start_time=$(date +%s%N)
/app/dist/desktop &
PID=$!

# Wait for window to appear
window_id=""
max_retries=100
for ((i=0; i<$max_retries; i++)); do
    window_id=$(xdotool search --onlyvisible --name "Lean Editor" 2>/dev/null | head -n 1 || true)
    if [ ! -z "$window_id" ]; then
        break
    fi
    # sleep for 10ms
    sleep 0.01
done

end_time=$(date +%s%N)

if [ -z "$window_id" ]; then
    echo "Error: Window never appeared."
    kill $PID 2>/dev/null || true
    exit 1
fi

duration=$(( (end_time - start_time) / 1000000 ))
echo "Startup duration (to window visible): $duration ms"

# Clean up
kill $PID 2>/dev/null || true
wait $PID 2>/dev/null || true
EOF

chmod +x /tmp/run_in_xvfb.sh
xvfb-run -a /tmp/run_in_xvfb.sh
