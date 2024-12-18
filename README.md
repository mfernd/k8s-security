# k8s-security

- [Kubernetes (k3d)](#kubernetes-k3d)
  - [Create k3d cluster with Istio](#create-k3d-cluster-with-istio)
  - [Deploy demo app (sentences generator)](#deploy-demo-app-sentences-generator)
- [How to dev - demo app](#how-to-dev---demo-app)
  - [Docker compose run](#docker-compose-run)
  - [Local run](#local-run)
  - [Configuration](#configuration)
    - [Common module](#common-module)
    - [Aggregator](#aggregator)
    - [Provider](#provider)

## Kubernetes (k3d)

Prerequisites:
- `kubectl` (obviously)
- [`k3d`](https://github.com/k3d-io/k3d)
- [`helmfile`](https://github.com/helmfile/helmfile)

### Create k3d cluster with Istio

Create k3d cluster (loadbalancer listening on ports 9080 and 9443, and without traefik):

```bash
just k3d-cluster-create
```

Install Istio (ambient mode), with kyverno, falco, (and more):

```bash
just k3d-cluster-init
```

### Deploy demo app (sentences generator)

```bash
just helm-demo-app-install
```

Go to [localhost:9443/](http://localhost:9443/) to see the app.

## How to dev - demo app

### Docker compose run

```bash
just up
```

### Local run

```bash
just dev
```

With [`mprocs`](https://github.com/pvolok/mprocs) to execute services in parallel.

### Configuration

#### Common module

Source: [`crates/common/`](./crates/common/)

| Env Var    | Description      | Default                    |
| ---------- | ---------------- | -------------------------- |
| `APP_HOST` | Application host | `"0.0.0.0"`                |
| `APP_PORT` | Application port | `3000` (or `80` in Docker) |

Used by all other modules.

#### Aggregator

Source: [`crates/aggregator-svc/`](./crates/aggregator-svc/)

| Env Var              | Description                                                                                                                | Default |
| -------------------- | -------------------------------------------------------------------------------------------------------------------------- | ------- |
| `APP_WORKERS_CONFIG` | Used to know where to get words (see [`workers_config.example.toml`](./crates/aggregator-svc/workers_config.example.toml)) | N/A     |

#### Provider

Source: [`crates/provider-svc/`](./crates/provider-svc/)

| Env Var             | Description                                                                                                              | Default |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------ | ------- |
| `APP_PROVIDER_KIND` | Define the provider type of the instance (see [struct `WordKind`](./crates/common/src/word_kind.rs) for possible values) | N/A     |
