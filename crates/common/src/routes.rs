/// AGGREGATOR routes
pub mod aggregator {
    /// To get a random generated sentence from the providers.
    pub const SENTENCE_ROUTE: &str = "/sentence";
}

/// PROVIDER routes
pub mod provider {
    /// To get the `WordKind` (type) of the provider.
    pub const KIND_ROUTE: &str = "/kind";
    /// To get a random word from the dictionaries.
    pub const RANDOM_WORD_ROUTE: &str = "/random_word";
}
