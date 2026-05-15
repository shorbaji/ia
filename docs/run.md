# Run

`ia run` starts a workload on insaali. Each workload type is its own subcommand. The first is `simeval`.

## simeval

Runs one episode of a policy in a simulator.

```sh
ia run simeval --policy hf://<owner>/<repo> [--sim HalfCheetah-v5] [--max-steps 100]
```

| flag          | required | default            | description                                      |
| ------------- | -------- | ------------------ | ------------------------------------------------ |
| `--policy`    | yes      | —                  | HuggingFace reference `hf://<owner>/<repo>`      |
| `--sim`       | no       | `HalfCheetah-v5`   | Gymnasium env id                                 |
| `--max-steps` | no       | `100`              | Maximum environment steps per episode            |

Examples:

```sh
ia run simeval --policy hf://acme/policy
ia run simeval --policy hf://acme/policy --sim HalfCheetah-v5 --max-steps 200
```

The command prints the new run id and its initial status. Use [`ia status`](reference.md) and `ia logs` to follow along.
