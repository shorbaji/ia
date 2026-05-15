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

The command prints the new run id and its initial status, then returns. The run continues on the compute backend.

```text
$ ia run simeval --policy hf://acme/policy
3f0b7c1e-4a8e-4d6e-9f2b-1b3c0e7f1d2a
status: running
```

## Following a run

`ia status <run-id>` and `ia logs <run-id>` fetch the live state from the compute backend on each call — nothing is mirrored.

```text
$ ia status 3f0b7c1e-4a8e-4d6e-9f2b-1b3c0e7f1d2a
id:        3f0b7c1e-4a8e-4d6e-9f2b-1b3c0e7f1d2a
simulator: HalfCheetah-v5
policy:    hf://acme/policy
status:    succeeded

$ ia logs 3f0b7c1e-4a8e-4d6e-9f2b-1b3c0e7f1d2a
[insaali] run_id=3f0b7c1e simulator=HalfCheetah-v5 policy_ref=hf://acme/policy
[insaali] step=1/100
[insaali] step=2/100
...
[insaali] done run_id=3f0b7c1e
```

## Lifecycle

| status      | meaning                                                            |
| ----------- | ------------------------------------------------------------------ |
| `running`   | The job is provisioning or executing on the compute backend.       |
| `succeeded` | The run finished without error.                                    |
| `failed`    | The run errored. `ia status` shows the message; `ia logs` has more.|
| `unknown`   | The backend could not be reached for a status call. Retry.         |

Status is whatever the compute backend reports at the moment of the call, lowercased. Once a run is `succeeded` or `failed`, it stays that way; backends retain logs for a limited window.
