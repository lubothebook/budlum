#!/usr/bin/env bash
# Testnet node initialization script.
# Usage: ./ops/scripts/testnet-init.sh [--data-dir DIR] [--chain-id ID]
set -euo pipefail

DATA_DIR="${DATA_DIR:-./data/testnet}"
CHAIN_ID="${CHAIN_ID:-45261}"
SECRETS_DIR="${DATA_DIR}/secrets"
GENESIS_FILE="${DATA_DIR}/genesis.json"
BINARY="cargo run --release --"

echo "=== Budlum Testnet Initialization ==="
echo "Data directory: ${DATA_DIR}"
echo "Chain ID: ${CHAIN_ID}"

# Parse arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --data-dir) DATA_DIR="$2"; SECRETS_DIR="${DATA_DIR}/secrets"; GENESIS_FILE="${DATA_DIR}/genesis.json"; shift 2 ;;
    --chain-id) CHAIN_ID="$2"; shift 2 ;;
    --binary) BINARY="$2"; shift 2 ;;
    *) echo "Unknown option: $1"; exit 1 ;;
  esac
done

# Create directories
mkdir -p "${SECRETS_DIR}"
mkdir -p "${DATA_DIR}/backups"
mkdir -p "${DATA_DIR}/snapshots"

# Generate validator key if not exists
if [ ! -f "${SECRETS_DIR}/validator.key" ]; then
  echo "Generating validator key..."
  ${BINARY} --gen-key "${SECRETS_DIR}/validator.key"
  echo "Validator key generated at: ${SECRETS_DIR}/validator.key"
else
  echo "Validator key already exists at: ${SECRETS_DIR}/validator.key"
fi

# Generate ed25519 identity key if not exists
if [ ! -f "${SECRETS_DIR}/node-id.key" ]; then
  echo "Generating node identity key..."
  ${BINARY} keygen --type ed25519 --output "${SECRETS_DIR}/node-id.key"
  echo "Node identity key generated at: ${SECRETS_DIR}/node-id.key"
else
  echo "Node identity key already exists at: ${SECRETS_DIR}/node-id.key"
fi

# Generate genesis if not exists
if [ ! -f "${GENESIS_FILE}" ]; then
  echo "Generating testnet genesis..."
  ${BINARY} genesis build \
    --chain-id "${CHAIN_ID}" \
    --output "${GENESIS_FILE}" \
    --dev-key-output "${SECRETS_DIR}/dev.key"
  echo "Genesis generated at: ${GENESIS_FILE}"
else
  echo "Genesis already exists at: ${GENESIS_FILE}"
fi

echo ""
echo "=== Initialization Complete ==="
echo ""
echo "To start the node:"
echo "  ${BINARY} \\"
echo "    --network testnet \\"
echo "    --genesis-file ${GENESIS_FILE} \\"
echo "    --validator-key-file ${SECRETS_DIR}/validator.key \\"
echo "    --db-path ${DATA_DIR}/budlum.db \\"
echo "    --rpc-enabled \\"
echo "    --port 5001"
