#!/bin/sh
#
# insaali CLI installer.
#
# Detects platform (Darwin arm64 / Linux x86_64 / Linux aarch64),
# downloads the matching prebuilt binary from the public GitHub
# Releases on shorbaji/ia, and installs it at ~/.local/bin/ia.
#
# Source-of-truth:    https://github.com/shorbaji/ia/blob/main/install.sh
# Verify this script: curl -fsSL https://insaali.com/install.sh.sha256 | shasum -a 256 -c -
#
# No data is collected. No root required. To uninstall: rm ~/.local/bin/ia.

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
