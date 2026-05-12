# Install

```sh
curl -fsSL https://insaali.com/install.sh | sh
```

## Build from source

```sh
git clone https://github.com/shorbaji/insaali
cd insaali/cli
cargo build --release
```

Requires a recent stable Rust toolchain.

## Sign in

```sh
ia login
```

Opens your browser to sign in (GitHub or Google), then stores a token at `~/.config/insaali/credentials` for subsequent commands.
