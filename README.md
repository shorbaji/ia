# ia

`ia` is the command-line client for [insaali](https://insaali.com) — simulation infrastructure for robotics.

## Install

```sh
curl -fsSL https://insaali.com/install.sh | sh
```

## Build from source

```sh
git clone https://github.com/shorbaji/ia
cd ia
cargo build --release
```

Requires a recent stable Rust toolchain.

## Sign in

```sh
ia login
```

Opens your browser to sign in (GitHub or Google), then stores a token at `~/.config/insaali/credentials` for subsequent commands.
