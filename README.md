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

## Run a simulation

```sh
ia run --sim mujoco --policy hf://owner/model
ia status <run-id>
ia logs <run-id>
```

`--compute-backend` defaults to `insaali`. Set `INSAALI_API_URL` to point the CLI at a non-production API.
