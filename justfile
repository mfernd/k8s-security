# default aliases
alias dev := start-mprocs

# docker aliases
alias up := start-docker
alias stop := stop-docker

# Shared environment variables
export APP_WORKERS_CONFIG := `cat ./crates/aggregator-svc/workers_config.example.toml`
export RUST_LOG := "tower_http=trace,info"

start-mprocs:
    cargo build
    # check if mprocs exists...
    @command -v mprocs > /dev/null || (echo "mprocs not found :(" && exit 1)
    # running services...
    @mprocs "APP_PORT=3000 cargo run --package aggregator-svc" \
           "APP_PORT=3001 APP_PROVIDER_KIND=adjective cargo run --bin provider-svc" \
           "APP_PORT=3002 APP_PROVIDER_KIND=noun cargo run --bin provider-svc" \
           "APP_PORT=3003 APP_PROVIDER_KIND=verb cargo run --bin provider-svc"

build-docker:
    docker build . --tag k8s-security/provider-svc --target provider-svc
    docker build . --tag k8s-security/aggregator-svc --target aggregator-svc

start-docker:
    docker compose up -d --remove-orphans

stop-docker:
    docker compose down --remove-orphans

helm_name := "my-k8s-security"

install-helm-chart:
    helm install {{ helm_name }} charts/mfernd-k8s-security/ --namespace {{ helm_name }} --create-namespace --wait
    kubectl label namespace {{ helm_name }} istio.io/dataplane-mode=ambient

uninstall-helm-chart:
    helm uninstall {{ helm_name }} --namespace {{ helm_name }} --wait
    kubectl delete ns/{{ helm_name }}

cluster_name := "mfernd-k8s-security"

k3d-cluster-create:
    # Start k3d cluster
    k3d cluster create {{ cluster_name }} -p "127.0.0.1:9080:80@loadbalancer" -p "127.0.0.1:9443:443@loadbalancer" --k3s-arg '--disable=traefik@server:*;agents:*'
    # Install Gateway API crds
    kubectl apply -f https://github.com/kubernetes-sigs/gateway-api/releases/download/v1.2.1/standard-install.yaml
    # Check if helmfile cli exists...
    @command -v helmfile > /dev/null || (echo "helmfile not found :(" && exit 1)
    # Install Istio on the cluster (in ambient mode)
    helmfile apply --file charts/helmfile.infra.yaml --wait

k3d-cluster-delete:
    k3d cluster delete {{ cluster_name }}
