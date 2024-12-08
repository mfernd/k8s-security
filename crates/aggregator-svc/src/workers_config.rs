use common::word_kind::WordKind;
use rand::{rngs::ThreadRng, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkersConfig {
    adjectives: Vec<String>,
    nouns: Vec<String>,
    verbs: Vec<String>,
}

impl WorkersConfig {
    pub fn from_toml(toml_config: &str) -> Result<Self, WorkersConfigError> {
        toml::from_str(toml_config).map_err(WorkersConfigError::InvalidTomlConfig)
    }

    pub fn get_rand_worker_by_kind(
        &self,
        word_kind: &WordKind,
    ) -> Result<String, WorkersConfigError> {
        // cached locally by rand, so don't need to provide it in parameters
        let mut rng = rand::thread_rng();

        match word_kind {
            WordKind::Adjective => Self::get_random_worker_from_vec(&mut rng, &self.adjectives),
            WordKind::Noun => Self::get_random_worker_from_vec(&mut rng, &self.nouns),
            WordKind::Verb => Self::get_random_worker_from_vec(&mut rng, &self.verbs),
        }
    }

    fn get_random_worker_from_vec(
        rng: &mut ThreadRng,
        workers: &Vec<String>,
    ) -> Result<String, WorkersConfigError> {
        workers.choose(rng).cloned().ok_or_else(|| {
            error!("no worker found in {:?}", workers);
            WorkersConfigError::WorkerNotFound
        })
    }
}

#[derive(Debug)]
pub enum WorkersConfigError {
    InvalidTomlConfig(toml::de::Error),
    WorkerNotFound,
}
