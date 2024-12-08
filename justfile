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
