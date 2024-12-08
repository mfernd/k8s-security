use aggregator_svc::{
    create_app,
    workers_config::{WorkersConfig, WorkersConfigError},
};
use common::app_service::{AppService, AppServiceError};

#[derive(Debug)]
pub enum AggregatorServiceError {
    StartServer(AppServiceError),
    MissingWorkerUrlsEnv(std::env::VarError),
    InvalidConfig(WorkersConfigError),
}

#[tokio::main]
async fn main() -> Result<(), AggregatorServiceError> {
    let workers_config = WorkersConfig::from_toml(
        std::env::var("APP_WORKERS_CONFIG")
            .map_err(AggregatorServiceError::MissingWorkerUrlsEnv)?
            .as_str(),
    )
    .map_err(AggregatorServiceError::InvalidConfig)?;

    let app = create_app(workers_config);
    let svc = AppService::new(app, None);

    let addr = format!(
        "{}:{}",
        std::env::var("APP_HOST").unwrap_or("0.0.0.0".into()),
        std::env::var("APP_PORT").unwrap_or("3000".into())
    );
    svc.run(&addr)
        .await
        .map_err(AggregatorServiceError::StartServer)
}
