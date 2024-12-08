use common::{
    app_service::{AppService, AppServiceError},
    word_kind::WordKind,
};
use provider_svc::create_app;

#[derive(Debug)]
pub enum ProviderServiceError {
    MissingProviderType(std::env::VarError),
    InvalidProviderType(String),
    StartServer(AppServiceError),
}

#[tokio::main]
async fn main() -> Result<(), ProviderServiceError> {
    let kind = get_provider_kind_env("APP_PROVIDER_KIND")?;
    let app = create_app(kind);
    let svc = AppService::new(app, None);

    let addr = format!(
        "{}:{}",
        std::env::var("APP_HOST").unwrap_or("0.0.0.0".into()),
        std::env::var("APP_PORT").unwrap_or("3000".into())
    );
    svc.run(&addr)
        .await
        .map_err(ProviderServiceError::StartServer)
}

fn get_provider_kind_env(kind_env_key: &str) -> Result<WordKind, ProviderServiceError> {
    let provider_type =
        std::env::var(kind_env_key).map_err(ProviderServiceError::MissingProviderType)?;

    WordKind::try_from(provider_type.as_str()).map_err(ProviderServiceError::InvalidProviderType)
}
