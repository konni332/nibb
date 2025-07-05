use crossterm::style::Stylize;
use crate::snippets::snippet::Snippet;

#[cfg(feature = "ansi")]
pub fn print_no_snippets_found() {
    println!("{}", "No snippets found".yellow());
}
#[cfg(not(feature = "ansi"))]
pub fn print_no_snippets_found() {
    println!("{}", "No snippets found");
}

#[cfg(feature = "ansi")]
pub fn print_snippet_list(snippets: &[&Snippet], verbose: bool) {
    if snippets.is_empty() {
        print_no_snippets_found();
        return;
    }
    println!("{}", "=== Snippets ===".bold());
    for snippet in snippets {
        snippet.pretty_print(verbose);
    }
}
#[cfg(not(feature = "ansi"))]
pub fn print_snippet_list(snippets: &[&Snippet], verbose: bool) {
    if snippets.is_empty() {
        print_no_snippets_found();
        return;
    }
    println!("{}", "=== Snippets ===");
    for snippet in snippets {
        snippet.pretty_print(verbose);
    }
}
