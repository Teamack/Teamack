#!/usr/bin/env bash
set -euo pipefail

# Build Linux app with Flutter and bundle as an AppImage using linuxdeploy.
APP_DIR="apps/desktop"
(cd "$APP_DIR" && flutter build linux)
# Placeholder packaging step
linuxdeploy --appdir "$APP_DIR/build/linux/x64/release/bundle" --output appimage
mv SOVR-x86_64.AppImage SOVR-linux-x64.AppImage
