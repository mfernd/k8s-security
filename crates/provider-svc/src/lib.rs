use axum::{extract::State, http::StatusCode, routing::get, Router};
use kind::ProviderKind;
use rand::seq::SliceRandom;

pub mod kind;

#[derive(Clone)]
struct AppState {
    kind: ProviderKind,
    words: Vec<String>,
}

pub fn create_app(kind: ProviderKind) -> Router {
    let words = kind.get_words();

    Router::new()
        .route("/kind", get(get_kind))
        .route("/random_word", get(random))
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
