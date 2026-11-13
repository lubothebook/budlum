#!/usr/bin/env bash
set -euo pipefail

# Testnet validator key generation script.
# Generates Ed25519, ML-DSA-65 (PQ), BLS, and VRF keys.
# Usage: ./testnet-keygen.sh [output_dir]

OUTPUT_DIR="${1:-$HOME/.budlum-testnet/keys}"
CHAIN_ID=45261

echo "=== Budlum Testnet Key Generation ==="
echo "Output dir: ${OUTPUT_DIR}"
echo "Chain ID:   ${CHAIN_ID}"

mkdir -p "${OUTPUT_DIR}"

if ! command -v bud &>/dev/null; then
    echo "ERROR: bud binary not found. Build with: cargo build --release"
    exit 1
fi

# Generate validator keys.
echo ""
echo "--- Generating validator keys ---"
bud keygen \
    --chain-id ${CHAIN_ID} \
    --output-dir "${OUTPUT_DIR}" \
    --pq-scheme ml-dsa-65

# Verify key files exist.
echo ""
echo "--- Verifying key files ---"
for keytype in validator bls vrf; do
    if [ -f "${OUTPUT_DIR}/${keytype}.key" ]; then
        echo "  OK: ${keytype}.key"
    else
        echo "  MISSING: ${keytype}.key"
    fi
done

# Show the validator address.
if [ -f "${OUTPUT_DIR}/validator.key" ]; then
    echo ""
    echo "--- Validator Address ---"
    bud keygen --show-address --key-file "${OUTPUT_DIR}/validator.key" 2>/dev/null || \
        echo "(address display requires --show-address support)"
fi

echo ""
echo "=== Key generation complete ==="
echo "IMPORTANT: Back up ${OUTPUT_DIR} securely. These keys control your validator stake."
