#!/usr/bin/env bash
# Testnet node health check script.
# Usage: ./ops/scripts/testnet-healthcheck.sh [--rpc-url URL]
set -euo pipefail

RPC_URL="${RPC_URL:-http://127.0.0.1:18545}"
METRICS_URL="${METRICS_URL:-http://127.0.0.1:19090/metrics}"
EXIT_CODE=0

echo "=== Budlum Testnet Health Check ==="
echo "RPC: ${RPC_URL}"
echo "Metrics: ${METRICS_URL}"
echo ""

# Parse arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --rpc-url) RPC_URL="$2"; shift 2 ;;
    --metrics-url) METRICS_URL="$2"; shift 2 ;;
    *) echo "Unknown option: $1"; exit 1 ;;
  esac
done

rpc_call() {
  local method="$1"
  local params="${2:-[]}"
  curl -sf "${RPC_URL}" -X POST \
    -H 'Content-Type: application/json' \
    -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"${method}\",\"params\":${params}}" 2>/dev/null
}

# Check RPC connectivity
echo "1. RPC Connectivity"
if response=$(rpc_call "bud_chainId"); then
  chain_id=$(echo "${response}" | python3 -I -c "import sys,json; r=json.load(sys.stdin); print(r.get('result','unknown'))" 2>/dev/null || echo "parse_error")
  echo "   OK Chain ID: ${chain_id}"
else
  echo "   FAIL Cannot connect to RPC"
  EXIT_CODE=1
fi

# Check block height
echo "2. Block Production"
if response=$(rpc_call "bud_blockNumber"); then
  height=$(echo "${response}" | python3 -I -c "import sys,json; r=json.load(sys.stdin); print(int(r.get('result','0x0'),16))" 2>/dev/null || echo "0")
  echo "   OK Block height: ${height}"
  if [ "${height}" -eq 0 ]; then
    echo "   WARN No blocks produced yet"
  fi
else
  echo "   FAIL Cannot get block number"
  EXIT_CODE=1
fi

# Check peer count
echo "3. P2P Network"
if response=$(rpc_call "bud_peerCount"); then
  peers=$(echo "${response}" | python3 -I -c "import sys,json; r=json.load(sys.stdin); print(int(r.get('result','0x0'),16))" 2>/dev/null || echo "0")
  echo "   OK Peer count: ${peers}"
  if [ "${peers}" -eq 0 ]; then
    echo "   WARN No peers connected"
  fi
else
  echo "   WARN Cannot get peer count (method may not be available)"
fi

# Check syncing status
echo "4. Sync Status"
if response=$(rpc_call "bud_syncing"); then
  syncing=$(echo "${response}" | python3 -I -c "import sys,json; r=json.load(sys.stdin); print(r.get('result',False))" 2>/dev/null || echo "unknown")
  echo "   OK Syncing: ${syncing}"
else
  echo "   WARN Cannot get sync status"
fi

# Check metrics endpoint
echo "5. Metrics"
if curl -sf "${METRICS_URL}" > /dev/null 2>&1; then
  echo "   OK Metrics endpoint accessible"
  # Check specific metrics
  if metrics=$(curl -sf "${METRICS_URL}" 2>/dev/null); then
    block_metric=$(echo "${metrics}" | grep -c "budlum_block_height" || true)
    peer_metric=$(echo "${metrics}" | grep -c "budlum_peer_count" || true)
    echo "   Block height metric: $([ "${block_metric}" -gt 0 ] && echo 'present' || echo 'missing')"
    echo "   Peer count metric: $([ "${peer_metric}" -gt 0 ] && echo 'present' || echo 'missing')"
  fi
else
  echo "   WARN Metrics endpoint not accessible"
fi

# Check Ethereum compatibility RPC
echo "6. Ethereum Compatibility RPC"
ETH_RPC="${ETH_RPC_URL:-http://127.0.0.1:18547}"
if response=$(curl -sf "${ETH_RPC}" -X POST \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","id":1,"method":"eth_chainId","params":[]}' 2>/dev/null); then
  eth_chain=$(echo "${response}" | python3 -I -c "import sys,json; r=json.load(sys.stdin); print(r.get('result','unknown'))" 2>/dev/null || echo "parse_error")
  echo "   OK eth_chainId: ${eth_chain}"
else
  echo "   WARN Ethereum RPC not accessible at ${ETH_RPC}"
fi

echo ""
echo "=== Health Check Complete (exit code: ${EXIT_CODE}) ==="
exit ${EXIT_CODE}
