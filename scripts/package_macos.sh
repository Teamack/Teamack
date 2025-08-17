#!/usr/bin/env bash
set -euo pipefail

# Build macOS app with Flutter and package into a DMG.
# Requires create-dmg or appdmg to be installed.
APP_DIR="apps/desktop"
(cd "$APP_DIR" && flutter build macos)
# Placeholder packaging step
create-dmg SOVR-macos-universal.dmg "$APP_DIR/build/macos/Build/Products/Release/SOVR.app"
