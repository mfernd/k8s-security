use axum::{extract::State, http::StatusCode, routing::get, Router};
use common::{routes::provider, word_kind::WordKind};
use rand::seq::SliceRandom;

pub mod words;

#[derive(Clone)]
struct AppState {
    kind: WordKind,
    words: Vec<String>,
}

pub fn create_app(kind: WordKind) -> Router {
    let words = words::get_words_by_kind(&kind);

    Router::new()
        .route(provider::KIND_ROUTE, get(get_kind))
        .route(provider::RANDOM_WORD_ROUTE, get(random))
        .with_state(AppState { kind, words })
}

async fn get_kind(State(state): State<AppState>) -> String {
    state.kind.to_string()
}

async fn random(State(state): State<AppState>) -> Result<String, (StatusCode, String)> {
    let mut rng = rand::thread_rng();

    state.words.choose(&mut rng).cloned().ok_or((
        StatusCode::NOT_FOUND,
        format!("Could not find word: \"{}\"", state.kind),
    ))
}
