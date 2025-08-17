#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(dirname "$(dirname "$SCRIPT_DIR")")"
OUTPUT="$ROOT_DIR/sovr-starter-pack.zip"

cd "$ROOT_DIR/docs/offline"
zip -r "$OUTPUT" .

echo "Created $OUTPUT"
