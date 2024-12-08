#[derive(Debug, Clone)]
pub enum WordKind {
    Adjective,
    Noun,
    Verb,
}

impl TryFrom<&str> for WordKind {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "adjective" | "adjectives" => Ok(Self::Adjective),
            "noun" | "nouns" => Ok(Self::Noun),
            "verb" | "verbs" => Ok(Self::Verb),
            invalid_value => Err(invalid_value.into()),
        }
    }
}

impl std::fmt::Display for WordKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            WordKind::Adjective => "adjective",
            WordKind::Noun => "noun",
            WordKind::Verb => "verb",
        };

        write!(f, "{}", str)
    }
}
