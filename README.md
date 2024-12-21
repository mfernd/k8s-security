# k8s-security

- [Kubernetes (kind)](#kubernetes-kind)
  - [Create cluster with Istio](#create-cluster-with-istio)
  - [Deploy demo app (sentences generator)](#deploy-demo-app-sentences-generator)
  - [Test the app](#test-the-app)
- [How to dev - demo app](#how-to-dev---demo-app)
  - [Docker compose run](#docker-compose-run)
  - [Local run](#local-run)
  - [Configuration](#configuration)
    - [Common module](#common-module)
    - [Aggregator](#aggregator)
    - [Provider](#provider)

## Kubernetes (kind)

Prerequisites:
- `kubectl` (obviously)
- [`kind`](https://github.com/kubernetes-sigs/kind)
- [`cloud-provider-kind`](https://github.com/kubernetes-sigs/cloud-provider-kind)
- [`helmfile`](https://github.com/helmfile/helmfile)

### Create cluster with Istio

Create `kind` cluster:

```bash
just kube-cluster-create
```

Install Istio (ambient mode), with kyverno, falco, (and more):

```bash
just kube-cluster-init
```

> [!NOTE]
> You will need to start a new shell where you will run `cloud-provider-kind` so that the LoadBalancer, the one created by the Gateway, gets an external IP.

### Deploy demo app (sentences generator)

```bash
just helm-demo-app-install
```

### Test the app

To get the external IP of our service:

```bash
export INGRESS_HOST=$(kubectl get -n my-sentences-demo-app gateway sentences-demo-app-gw -o jsonpath='{.status.addresses[0].value}')
echo $INGRESS_HOST
```

And then you try the app with:

```bash
curl http://$INGRESS_HOST/
```

You can check in Kiali, that the mTLS is enabled in our namespace:

```bash
kubectl port-forward -n istio-system svc/kiali 20001:20001
```

Go to [http://localhost:20001](http://localhost:20001), and see the app in Kiali:

![App In Kiali homepage](./docs/app_in_kiali.png)

And the traffic graph (you need to send some requests to the app to see it):

![App In Kiali traffic graph](./docs/traffic_graph_kiali.png)

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
