#!/usr/bin/env bash
set -euo pipefail

# Testnet data reset script.
# WARNING: This deletes all testnet data!
# Usage: ./testnet-reset.sh [data_dir]

DATA_DIR="${1:-$HOME/.budlum-testnet}"

echo "=== Budlum Testnet Reset ==="
echo "Data dir: ${DATA_DIR}"
echo ""

if [ ! -d "${DATA_DIR}" ]; then
    echo "Data directory does not exist. Nothing to reset."
    exit 0
fi

echo "WARNING: This will delete ALL testnet data in ${DATA_DIR}"
echo "Press Ctrl+C to cancel, or Enter to continue..."
read -r

# Create backup before reset.
BACKUP_DIR="${DATA_DIR}.backup.$(date +%Y%m%d_%H%M%S)"
echo "Creating backup at ${BACKUP_DIR}..."
cp -r "${DATA_DIR}" "${BACKUP_DIR}"

# Remove data.
echo "Removing testnet data..."
rm -rf "${DATA_DIR}/db"
rm -rf "${DATA_DIR}/blocks"
rm -rf "${DATA_DIR}/state"

# Keep keys and config.
echo "Preserved: keys/ and config files"

echo ""
echo "=== Reset complete ==="
echo "Backup at: ${BACKUP_DIR}"
echo "Run testnet-init.sh to reinitialize."
