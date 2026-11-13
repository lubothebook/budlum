#!/usr/bin/env bash
set -euo pipefail

# Testnet database backup script.
# Usage: ./testnet-backup.sh [data_dir] [backup_dir]

DATA_DIR="${1:-$HOME/.budlum-testnet}"
BACKUP_DIR="${2:-$HOME/.budlum-backups}"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_NAME="testnet_backup_${TIMESTAMP}"
BACKUP_PATH="${BACKUP_DIR}/${BACKUP_NAME}"

echo "=== Budlum Testnet Backup ==="
echo "Data dir:   ${DATA_DIR}"
echo "Backup dir: ${BACKUP_PATH}"

if [ ! -d "${DATA_DIR}" ]; then
    echo "ERROR: Data directory does not exist: ${DATA_DIR}"
    exit 1
fi

mkdir -p "${BACKUP_DIR}"

# Use the built-in backup command if available.
if command -v bud &>/dev/null; then
    echo "Using bud backup command..."
    bud backup \
        --data-dir "${DATA_DIR}" \
        --output "${BACKUP_PATH}" \
        --compress
    echo "Backup completed: ${BACKUP_PATH}"
else
    echo "bud binary not found, falling back to tar..."
    tar czf "${BACKUP_PATH}.tar.gz" -C "$(dirname "${DATA_DIR}")" "$(basename "${DATA_DIR}")"
    echo "Backup completed: ${BACKUP_PATH}.tar.gz"
fi

# Verify backup integrity.
echo "Verifying backup..."
if [ -d "${BACKUP_PATH}" ]; then
    FILE_COUNT=$(find "${BACKUP_PATH}" -type f | wc -l)
    echo "Backup contains ${FILE_COUNT} files"
elif [ -f "${BACKUP_PATH}.tar.gz" ]; then
    tar tzf "${BACKUP_PATH}.tar.gz" | wc -l | xargs -I{} echo "Archive contains {} entries"
fi

# Retention: keep last 7 backups.
echo "Applying retention policy (keep last 7)..."
ls -dt "${BACKUP_DIR}"/testnet_backup_* 2>/dev/null | tail -n +8 | while read -r old; do
    echo "  Removing old backup: ${old}"
    rm -rf "${old}"
done

echo "=== Backup complete ==="
