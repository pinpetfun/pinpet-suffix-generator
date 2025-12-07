#!/bin/bash

# API Performance Benchmark Script
# Tests concurrent API calls to verify zero-latency

echo "================================"
echo "API Performance Benchmark"
echo "================================"
echo ""

API_URL="http://localhost:5057/api/v1/pet/address"
STATUS_URL="http://localhost:5057/api/v1/pet/status"

# Check if server is running
echo "[1] Checking server status..."
if ! curl -s -f "$STATUS_URL" > /dev/null; then
    echo "❌ Server is not running. Please start the server first:"
    echo "   cargo run --release"
    exit 1
fi
echo "✅ Server is running"
echo ""

# Warmup
echo "[2] Warming up (5 requests)..."
for i in {1..5}; do
    curl -s "$API_URL" > /dev/null
done
echo "✅ Warmup complete"
echo ""

# Test 1: Sequential requests with timing
echo "[3] Testing sequential requests (10 requests)..."
TOTAL_TIME=0
for i in {1..10}; do
    START=$(date +%s%N)
    RESPONSE=$(curl -s "$API_URL")
    END=$(date +%s%N)
    DURATION=$((($END - $START) / 1000000)) # Convert to milliseconds
    TOTAL_TIME=$(($TOTAL_TIME + $DURATION))
    echo "  Request $i: ${DURATION}ms"
done
AVG_TIME=$(($TOTAL_TIME / 10))
echo "✅ Average response time: ${AVG_TIME}ms"
echo ""

# Test 2: Parallel requests
echo "[4] Testing parallel requests (50 concurrent)..."
START_PARALLEL=$(date +%s%N)
for i in {1..50}; do
    curl -s "$API_URL" > /dev/null &
done
wait
END_PARALLEL=$(date +%s%N)
PARALLEL_DURATION=$((($END_PARALLEL - $START_PARALLEL) / 1000000))
echo "✅ 50 concurrent requests completed in: ${PARALLEL_DURATION}ms"
echo ""

# Test 3: High load burst
echo "[5] Testing high load burst (100 concurrent)..."
START_BURST=$(date +%s%N)
for i in {1..100}; do
    curl -s "$API_URL" > /dev/null &
done
wait
END_BURST=$(date +%s%N)
BURST_DURATION=$((($END_BURST - $START_BURST) / 1000000))
echo "✅ 100 concurrent requests completed in: ${BURST_DURATION}ms"
echo ""

# Test 4: Check pool status
echo "[6] Checking address pool status..."
STATUS=$(curl -s "$STATUS_URL")
echo "$STATUS" | jq '.'
echo ""

# Summary
echo "================================"
echo "Performance Summary"
echo "================================"
echo "Sequential avg:    ${AVG_TIME}ms per request"
echo "50 concurrent:     ${PARALLEL_DURATION}ms total"
echo "100 concurrent:    ${BURST_DURATION}ms total"
echo ""

if [ $AVG_TIME -lt 10 ]; then
    echo "✅ EXCELLENT: Sub-10ms average response time!"
elif [ $AVG_TIME -lt 50 ]; then
    echo "✅ GOOD: Sub-50ms average response time"
else
    echo "⚠️  WARNING: Response time may need optimization"
fi

echo ""
echo "================================"
