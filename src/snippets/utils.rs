use chrono::NaiveDate;
use crate::snippets::snippet::Snippet;

pub fn filter_snippets(snippets: Vec<Snippet>, filter: &str) -> Vec<Snippet> {
    if filter.trim().is_empty() || filter == "all" || filter == "a"{
        return snippets;
    }

    let mut filtered = Vec::new();

    let parsed_date = NaiveDate::parse_from_str(filter, "%Y-%m-%d").ok();

    for snippet in snippets {
        let meta = &snippet.meta;
        let matches_text =
            meta.language.to_string().contains(filter)
            || meta.name.contains(filter)
            || meta.description.contains(filter)
            || meta.tags.contains(&filter.to_string());

        let matches_date = if let Some(date) = parsed_date {
            meta.created.date_naive() <= date
        }
        else {
            false
        };
        if matches_text || matches_date {
            filtered.push(snippet);
        }
    }
    filtered
}