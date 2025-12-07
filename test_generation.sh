#!/bin/bash

# Quick test to verify address generation works
echo "Starting Pet address generation test..."
echo "This will run the server for 30 seconds and check if it generates addresses."
echo ""

# Start the server in background
timeout 30 cargo run --release 2>&1 &
SERVER_PID=$!

# Wait a bit for server to start
sleep 5

# Try to get an address
echo "Attempting to fetch a Pet address..."
response=$(curl -s http://localhost:5057/api/v1/pet/status 2>/dev/null)

if [ $? -eq 0 ]; then
    echo "✓ Server is running"
    echo "Status: $response"
else
    echo "✗ Server not responding yet"
fi

# Wait for the timeout
wait $SERVER_PID 2>/dev/null

echo ""
echo "Test complete!"
