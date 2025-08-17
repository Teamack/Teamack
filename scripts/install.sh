#!/usr/bin/env bash
set -euo pipefail
VERSION="${1:-latest}"
OS="$(uname -s)"
ARCH="$(uname -m)"

if [ "$VERSION" = "latest" ]; then
  VERSION="$(curl -s https://api.github.com/repos/YOURORG/sovr/releases/latest | grep tag_name | cut -d '"' -f4)"
fi

dl() {
  url="$1"
  echo "Downloading $url"
  curl -L --fail --retry 3 -o "$2" "$url"
}

case "$OS-$ARCH" in
  Darwin-arm64|Darwin-x86_64)
    ASSET="SOVR-macos-universal.dmg"
    dl "https://github.com/YOURORG/sovr/releases/download/$VERSION/$ASSET" "$ASSET"
    echo "Open $ASSET to install."
    ;;
  Linux-x86_64)
    ASSET="SOVR-linux-x64.AppImage"
    dl "https://github.com/YOURORG/sovr/releases/download/$VERSION/$ASSET" "$ASSET"
    chmod +x "$ASSET"
    echo "Run with ./$ASSET"
    ;;
  Linux-aarch64|Linux-arm64)
    ASSET="sovr-headless-linux-aarch64.tar.gz"
    dl "https://github.com/YOURORG/sovr/releases/download/$VERSION/$ASSET" "$ASSET"
    tar xzf "$ASSET"
    echo "Run headless with ./sovr-headless mesh join --mdns"
    ;;
  *)
    echo "Unsupported platform: $OS $ARCH" >&2; exit 1;;
esac
