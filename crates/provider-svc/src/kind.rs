pub const ADJECTIVES: &str = include_str!("../../../.data/adjectives.txt");
pub const NOUNS: &str = include_str!("../../../.data/nouns.txt");
pub const VERBS: &str = include_str!("../../../.data/verbs.txt");

#[derive(Debug, Clone)]
pub enum ProviderKind {
    Adjectives,
    Nouns,
    Verbs,
}

impl ProviderKind {
    pub fn get_words(&self) -> Vec<String> {
        match self {
            ProviderKind::Adjectives => split_by_lines(ADJECTIVES),
            ProviderKind::Nouns => split_by_lines(NOUNS),
            ProviderKind::Verbs => split_by_lines(VERBS),
        }
    }
}

impl TryFrom<&str> for ProviderKind {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "adjective" | "adjectives" => Ok(Self::Adjectives),
            "noun" | "nouns" => Ok(Self::Nouns),
            "verb" | "verbs" => Ok(Self::Verbs),
            invalid_value => Err(invalid_value.into()),
        }
    }
}

impl std::fmt::Display for ProviderKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn split_by_lines(data: &str) -> Vec<String> {
    data.lines().map(|str| str.into()).collect()
}
