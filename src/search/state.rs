use crate::{
    core::{app::App, mode::Mode},
    search::matcher,
};

pub fn enter_search_mode(app: &mut App) {
    app.mode = Mode::Search;
    app.search_query.clear();
    apply_filter(app);
}

pub fn push_char(app: &mut App, ch: char) {
    app.search_query.push(ch);
    apply_filter(app);
}

pub fn backspace(app: &mut App) {
    app.search_query.pop();
    apply_filter(app);
}

pub fn apply_filter(app: &mut App) {
    if app.search_query.is_empty() {
        app.filtered = app.full_entries.clone();
    } else {
        app.filtered = app
            .full_entries
            .iter()
            .filter(|entry| matcher::fuzzy_match(&entry.name, &app.search_query))
            .cloned()
            .collect();
    }

    app.selected = 0;
    app.scroll = 0;
}

pub fn cancel_search(app: &mut App) {
    app.search_query.clear();
    app.filtered = app.full_entries.clone();
    app.selected = 0;
    app.scroll = 0;
    app.mode = Mode::Normal;
}

pub fn submit_search(app: &mut App) {
    app.mode = Mode::Normal;
    app.status = format!(
        "search applied: '{}' ({} matches)",
        app.search_query,
        app.filtered.len()
    );
}
