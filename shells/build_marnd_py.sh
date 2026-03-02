#!/usr/bin/env bash
set -uo pipefail

# # From anywhere, run from repo root
# REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# cd "$REPO_ROOT"

# Optional: choose a specific python (uncomment if you want)
# PYTHON=python3.11
# "$PYTHON" -m pip install -U maturin

python -m pip install -U maturin

maturin develop -m crates/marnd_py/Cargo.toml
if [ $? -eq 0 ]; then
    echo "✅ marnd_py built and installed into current Python environment."
else
    echo "❌ Build failed."
    exit 1
fi

python python/tests/test_marnd_py.py
if [ $? -eq 0 ]; then
    echo "✅ Sanity check python package marnd_py OK!."
else
    echo "❌ Python test failed."
    exit 1
fi
