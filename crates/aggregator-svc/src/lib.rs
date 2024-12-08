use axum::{extract::State, routing::get, Router};
use common::{
    routes::{aggregator, provider},
    word_kind::WordKind,
};
use futures::{stream, StreamExt};
use tracing::{error, info};
use workers_config::WorkersConfig;

pub mod workers_config;

#[derive(Clone)]
struct AppState {
    workers_config: WorkersConfig,
}

pub fn create_app(workers_config: WorkersConfig) -> Router {
    Router::new()
        .route(aggregator::SENTENCE_ROUTE, get(sentence_handler))
        .with_state(AppState { workers_config })
}

async fn sentence_handler(State(state): State<AppState>) -> String {
    let client = reqwest::Client::new();

    let sentence_order = [
        WordKind::Adjective,
        WordKind::Noun,
        WordKind::Verb,
        WordKind::Adjective,
        WordKind::Noun,
    ];

    stream::iter(sentence_order)
        .map(|kind| fetch_word(&client, &state.workers_config, kind))
        .fold(String::new(), |acc, x| async move {
            let word = x.await.unwrap_or(" - ".into());
            if acc.is_empty() {
                return word;
            }
            acc + " " + &word.to_lowercase()
        })
        .await
}

#[derive(Debug)]
enum FetchWordError {
    CouldNotRetrieveWorkerUrl,
    FetchWordFromWorker,
    FailedToDecodeResponse,
}

async fn fetch_word(
    client: &reqwest::Client,
    workers: &WorkersConfig,
    word_kind: WordKind,
) -> Result<String, FetchWordError> {
    let mut url = workers
        .get_rand_worker_by_kind(&word_kind)
        .map_err(|_| FetchWordError::CouldNotRetrieveWorkerUrl)?;
    if !url.starts_with("http://") && !url.starts_with("https://") {
        url.insert_str(0, "http://");
    };

    url.push_str(provider::RANDOM_WORD_ROUTE);

    info!("fetching worker \"{}\" for \"{}\"", url, &word_kind);
    client
        .get(url)
        .send()
        .await
        .map_err(|err| {
            error!("could not fetch word from worker, because: {:?}", err);
            FetchWordError::FetchWordFromWorker
        })?
        .text()
        .await
        .map_err(|err| {
            // should happen rarely
            error!("could not decode response body, because: {:?}", err);
            FetchWordError::FailedToDecodeResponse
        })
}
