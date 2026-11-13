#!/usr/bin/env bash
set -euo pipefail

# Testnet continuous monitoring script.
# Polls node health and prints status every INTERVAL seconds.
# Usage: ./testnet-monitor.sh [rpc_url] [interval_secs]

RPC_URL="${1:-http://127.0.0.1:18545}"
INTERVAL="${2:-30}"
ETH_URL="${RPC_URL%:*}:18547"

echo "=== Budlum Testnet Monitor ==="
echo "RPC:      ${RPC_URL}"
echo "ETH RPC:  ${ETH_URL}"
echo "Interval: ${INTERVAL}s"
echo ""

check_rpc() {
    local response
    response=$(curl -s -m 5 "${RPC_URL}/health" 2>/dev/null) || true
    if [ -n "${response}" ]; then
        echo "RPC: UP"
    else
        echo "RPC: DOWN"
    fi
}

check_block_height() {
    local height
    height=$(curl -s -m 5 "${RPC_URL}/status" 2>/dev/null | \
        python3 -c "import sys,json; print(json.load(sys.stdin).get('block_height', 'N/A'))" 2>/dev/null) || true
    echo "Block: ${height:-N/A}"
}

check_peers() {
    local peers
    peers=$(curl -s -m 5 "${RPC_URL}/net/peers" 2>/dev/null | \
        python3 -c "import sys,json; d=json.load(sys.stdin); print(len(d.get('peers', d.get('connected', []))))" 2>/dev/null) || true
    echo "Peers: ${peers:-N/A}"
}

check_eth_rpc() {
    local result
    result=$(curl -s -m 5 -X POST -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' \
        "${ETH_URL}" 2>/dev/null | \
        python3 -c "import sys,json; print(json.load(sys.stdin).get('result', 'N/A'))" 2>/dev/null) || true
    echo "ETH Block: ${result:-N/A}"
}

check_sync() {
    local syncing
    syncing=$(curl -s -m 5 "${RPC_URL}/status" 2>/dev/null | \
        python3 -c "import sys,json; print(json.load(sys.stdin).get('syncing', 'N/A'))" 2>/dev/null) || true
    echo "Syncing: ${syncing:-N/A}"
}

while true; do
    TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')
    echo "--- ${TIMESTAMP} ---"
    check_rpc
    check_block_height
    check_peers
    check_eth_rpc
    check_sync
    echo ""
    sleep "${INTERVAL}"
done
