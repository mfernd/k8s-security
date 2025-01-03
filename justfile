# default aliases
alias dev := start-mprocs

# docker aliases
alias up := start-docker
alias stop := stop-docker

# Shared environment variables
export APP_WORKERS_CONFIG := `cat ./crates/aggregator-svc/workers_config.example.toml`
export RUST_LOG := "tower_http=trace,info"

# --- Dev environment

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

load-http ip="localhost" port="3000":
    # check if vegeta exists...
    @command -v vegeta > /dev/null || (echo "vegeta not found :(" && exit 1)
    echo "GET http://{{ ip }}:{{ port }}/" | vegeta attack -name=100qps -rate=100 -duration=10s > results.bin
    # @vegeta report -type=json results.bin > metrics.json
    @cat results.bin | vegeta plot > plot.html
    @cat results.bin | vegeta report -type="hist[0,1ms,2ms,3ms,4ms,5ms,6ms,7ms,8ms]"
    @open plot.html

# --- Kubernetes

cluster_name := "mfernd-k8s-security"
helm_name := "my-sentences-demo-app"

custom_yaml_folder := "./charts/kubernetes_yaml"
kyv_policy_folder := "./charts/kubernetes_yaml/kyverno"
kyv_tests_folder := "./charts/kubernetes_yaml/kyverno/tests"
falco_folder := "./charts/kubernetes_yaml/falco"

kube-cluster-create:
    kind create cluster --name {{ cluster_name }}
    kubectl label node {{ cluster_name }}-control-plane node.kubernetes.io/exclude-from-external-load-balancers-
    # Install Gateway API crds
    kubectl apply -f {{ custom_yaml_folder }}/crds_gateway.networking.k8s.io_v1.2.1.yaml

kube-cluster-init:
    # Check if helmfile cli exists...
    @command -v helmfile > /dev/null || (echo "helmfile not found :(" && exit 1)
    # Install Istio on the cluster (in ambient mode)
    helmfile apply --file ./charts/helmfile.infra.yaml --wait
    # For Istio and Kiali dashboard, for demonstration purposes only
    kubectl apply -f {{ custom_yaml_folder }}/istio_addon_prometheus_v1.24.yaml
    kubectl apply -f {{ custom_yaml_folder }}/istio_addon_grafana_v1.24.yaml
    # Add Kyverno policies
    kubectl apply -f {{ kyv_policy_folder }}

kube-cluster-delete:
    kind delete cluster --name {{ cluster_name }}

helm-demo-app-install:
    helmfile apply --file ./charts/helmfile.apps.yaml

helm-demo-app-uninstall:
    helmfile destroy --file ./charts/helmfile.apps.yaml
    kubectl delete namespace {{ helm_name }}

kyverno-valid-tests:
    kyverno apply {{ kyv_policy_folder }}/check-images.cpol.yaml --resource {{ kyv_tests_folder }}/valid-pod.yaml
    kyverno apply {{ kyv_policy_folder }}/require-requests-limits.cpol.yaml --resource {{ kyv_tests_folder }}/valid-pod.yaml
    kyverno apply {{ kyv_policy_folder }}/restrict-nodeport.cpol.yaml --resource {{ kyv_tests_folder }}/valid-svc-clusterip.yaml

kyverno-invalid-tests:
    ! kyverno apply {{ kyv_policy_folder }}/check-images.cpol.yaml --resource {{ kyv_tests_folder }}/invalid-pod-image.yaml
    ! kyverno apply {{ kyv_policy_folder }}/require-requests-limits.cpol.yaml --resource {{ kyv_tests_folder }}/invalid-pod-requests-limits.yaml
    ! kyverno apply {{ kyv_policy_folder }}/restrict-nodeport.cpol.yaml --resource {{ kyv_tests_folder }}/invalid-svc-nodeport.yaml

kyverno-mutate-tests:
    kyverno apply {{ kyv_policy_folder }}/add-default-securitycontext.cpol.yaml --resource {{ kyv_tests_folder }}/mutate-pod-security-context.yaml
    kyverno apply {{ kyv_policy_folder }}/add-istio-mesh-ns.cpol.yaml --resource {{ kyv_tests_folder }}/mutate-ns-istio-mesh.yaml

falco-trigger-rule:
    # creating resources that escapes the ClusterPolicies of Kyverno...
    @kubectl apply -f {{ falco_folder }}/trigger-rule.yaml
    @kubectl wait -n trigger-falco-rule pod/bayrou --for=condition=Ready
    # trigger rule by reading sensitive file
    kubectl exec -n trigger-falco-rule -it pod/bayrou -- cat /etc/shadow
    # reading latest Falco warnings
    kubectl logs -l app.kubernetes.io/name=falco -n falco -c falco | grep Warning
    # removing created resources for triggering a rule...
    @kubectl delete -f {{ falco_folder }}/trigger-rule.yaml
