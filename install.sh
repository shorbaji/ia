#!/bin/sh
set -e

OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS-$ARCH" in
  darwin-arm64)   TARGET="aarch64-apple-darwin" ;;
  linux-x86_64)   TARGET="x86_64-unknown-linux-musl" ;;
  linux-aarch64)  TARGET="aarch64-unknown-linux-musl" ;;
  *)
    echo "unsupported platform: $OS-$ARCH" >&2
    exit 1
    ;;
esac

INSTALL_DIR="${HOME}/.local/bin"
mkdir -p "$INSTALL_DIR"

URL="https://github.com/shorbaji/ia/releases/latest/download/ia-$TARGET"
echo "downloading $URL"
curl -fsSL "$URL" -o "$INSTALL_DIR/ia"
chmod +x "$INSTALL_DIR/ia"

echo "installed: $INSTALL_DIR/ia"

case ":$PATH:" in
  *":$INSTALL_DIR:"*) ;;
  *)
    echo
    echo "$INSTALL_DIR is not on your PATH. Add this to your shell rc:"
    echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
    ;;
esac
