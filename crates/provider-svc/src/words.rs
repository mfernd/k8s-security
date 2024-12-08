use common::word_kind::WordKind;

pub const ADJECTIVES: &str = include_str!("../../../.data/adjectives.txt");
pub const NOUNS: &str = include_str!("../../../.data/nouns.txt");
pub const VERBS: &str = include_str!("../../../.data/verbs.txt");

pub fn get_words_by_kind(word_kind: &WordKind) -> Vec<String> {
    match word_kind {
        WordKind::Adjective => split_by_lines(ADJECTIVES),
        WordKind::Noun => split_by_lines(NOUNS),
        WordKind::Verb => split_by_lines(VERBS),
    }
}

fn split_by_lines(data: &str) -> Vec<String> {
    data.lines().map(|str| str.into()).collect()
}
