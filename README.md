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

## Authentication

```sh
ia login    # browser sign-in (GitHub or Google); writes ~/.config/insaali/credentials
ia logout   # remove the saved token
```

## Run a simeval

```sh
ia run simeval --backend anyscale --policy hf://owner/model
ia run simeval --backend k8s      --policy hf://owner/model --sim HalfCheetah-v5 --max-steps 200
ia status <run-id>
ia logs <run-id>
```

`--backend` selects where the run executes: `anyscale` (Anyscale Job) or `k8s` (KubeRay RayJob on the insaali GKE cluster). `--sim` defaults to `HalfCheetah-v5`; `--max-steps` defaults to 100. Set `INSAALI_API_URL` to point the CLI at a non-production API.
