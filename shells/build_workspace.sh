#!/bin/evn bash

set -uo pipefail

cargo build --workspace --exclude marcrypto_py
if [ $? -eq 0 ]; then
    echo "✅ The workspace is built successfully."
else
    echo "❌ Build failed."
    exit 1
fi

SCRIPT_DIR="$(dirname "$0")"
echo "Script location: $SCRIPT_DIR"

bash "${SCRIPT_DIR}/build_marcrypto_py.sh"
if [ $? -eq 0 ]; then
    echo "✅ The marcrypto_py is built successfully."
else
    echo "❌ Build failed."
    exit 1
fi