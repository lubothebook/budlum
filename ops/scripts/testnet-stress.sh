#!/usr/bin/env bash
set -euo pipefail

# Testnet stress test: sends many transactions to measure throughput.
# Usage: ./testnet-stress.sh [rpc_url] [num_transactions] [concurrency]

RPC_URL="${1:-http://127.0.0.1:18545}"
NUM_TX="${2:-1000}"
CONCURRENCY="${3:-10}"

echo "=== Budlum Testnet Stress Test ==="
echo "RPC:         ${RPC_URL}"
echo "Transactions: ${NUM_TX}"
echo "Concurrency:  ${CONCURRENCY}"
echo ""

if ! command -v bud &>/dev/null; then
    echo "ERROR: bud binary not found."
    exit 1
fi

# Check node is up.
if ! curl -s -m 5 "${RPC_URL}/health" &>/dev/null; then
    echo "ERROR: Node not reachable at ${RPC_URL}"
    exit 1
fi

START_HEIGHT=$(curl -s "${RPC_URL}/status" | python3 -c "import sys,json; print(json.load(sys.stdin).get('block_height', 0))" 2>/dev/null || echo "0")
START_TIME=$(date +%s)

echo "Start height: ${START_HEIGHT}"
echo "Sending ${NUM_TX} transactions..."

# Parallel transaction sending.
seq 1 "${NUM_TX}" | xargs -P "${CONCURRENCY}" -I{} \
    curl -s -X POST -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"bdlm_sendTransaction","params":[{"type":"transfer","to":"0000000000000000000000000000000000000001","amount":1}],"id":{}}' \
    "${RPC_URL}" -o /dev/null 2>/dev/null || true

END_TIME=$(date +%s)
ELAPSED=$((END_TIME - START_TIME))

# Wait for blocks to be produced.
sleep 10

END_HEIGHT=$(curl -s "${RPC_URL}/status" | python3 -c "import sys,json; print(json.load(sys.stdin).get('block_height', 0))" 2>/dev/null || echo "0")
BLOCKS_PRODUCED=$((END_HEIGHT - START_HEIGHT))

echo ""
echo "=== Results ==="
echo "Time elapsed: ${ELAPSED}s"
echo "Blocks produced: ${BLOCKS_PRODUCED}"
echo "TX submitted: ${NUM_TX}"
if [ "${ELAPSED}" -gt 0 ]; then
    TPS=$((NUM_TX / ELAPSED))
    echo "Throughput: ~${TPS} tx/s"
fi
echo "=== Stress test complete ==="
