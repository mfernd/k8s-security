# k8s-security

## Docker compose run

```bash
just up
```

## Local run

```bash
just dev
```

With [`mprocs`](https://github.com/pvolok/mprocs) to execute services in parallel.

## Configuration

### Common module

Source: [`crates/common/`](./crates/common/)

| Env Var    | Description      | Default                    |
| ---------- | ---------------- | -------------------------- |
| `APP_HOST` | Application host | `"0.0.0.0"`                |
| `APP_PORT` | Application port | `3000` (or `80` in Docker) |

Used by all other modules.

### Aggregator

Source: [`crates/aggregator-svc/`](./crates/aggregator-svc/)

| Env Var              | Description                                                                                                                | Default |
| -------------------- | -------------------------------------------------------------------------------------------------------------------------- | ------- |
| `APP_WORKERS_CONFIG` | Used to know where to get words (see [`workers_config.example.toml`](./crates/aggregator-svc/workers_config.example.toml)) | N/A     |

### Provider

Source: [`crates/provider-svc/`](./crates/provider-svc/)

| Env Var             | Description                                                                                                              | Default |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------ | ------- |
| `APP_PROVIDER_KIND` | Define the provider type of the instance (see [struct `WordKind`](./crates/common/src/word_kind.rs) for possible values) | N/A     |
