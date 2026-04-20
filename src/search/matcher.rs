pub fn fuzzy_match(candidate: &str, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }

    candidate.to_lowercase().contains(&query.to_lowercase())
}
