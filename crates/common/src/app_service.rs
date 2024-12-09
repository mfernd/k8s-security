use axum::{routing::get, Router};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{error, info};

#[derive(Debug)]
pub enum AppServiceError {
    InvalidBindAddress(std::io::Error),
    ServerNotStarting(std::io::Error),
}

pub struct AppService {
    router: Router,
}

impl AppService {
    /// Create an [AppService], that later, can be [AppService::run].
    pub fn new(app_router: Router, worker_urls: Option<Vec<String>>) -> Self {
        let router = Router::new()
            .merge(app_router)
            .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
            .route(
                "/_health",
                get(|| async { health_handler(worker_urls).await }),
            )
            .layer(ServiceBuilder::new().layer(CorsLayer::permissive()));

        Self { router }
    }

    /// Run the [AppService] instance. It will move `self.router`, so it can't be used after.
    pub async fn run(self, addr: &str) -> Result<(), AppServiceError> {
        tracing_subscriber::fmt::init();

        info!("starting the server...");
        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .map_err(AppServiceError::InvalidBindAddress)?;
        info!("listening on http://{}/", addr);

        axum::serve(listener, self.router)
            .await
            .map_err(AppServiceError::ServerNotStarting)
    }
}

const HEALTHY_MSG: &str = "healthy <3";

/// Health handler to check the health of the application, will check the health of all its related workers.
/// If empty will just return "healthy".
async fn health_handler(worker_urls: Option<Vec<String>>) -> Result<String, String> {
    let worker_urls = match worker_urls {
        Some(urls) => urls,
        None => return Ok(HEALTHY_MSG.into()),
    };

    let client = reqwest::Client::new();
    let mut are_workers_alive = true;
    let mut unhealthy_workers: Vec<String> = Vec::new();

    // we check the health of all our workers
    for worker_url in worker_urls {
        let response = client.get(&worker_url).send().await;

        // small mapping for status & error :)
        let response_result = match response {
            Ok(response) => response.status().is_success(),
            Err(e) => {
                error!("could not send health request because: {}", e);
                false
            }
        };

        if response_result {
            unhealthy_workers.push(worker_url);
            // only update it if it's true, not necessary if not
            if are_workers_alive {
                are_workers_alive = false;
            }
        }
    }

    if are_workers_alive {
        return Ok(HEALTHY_MSG.into());
    }

    error!("unhealthy workers: {:?}", unhealthy_workers);

    Err("unhealthy 🤒, see the logs for more information".into())
}
