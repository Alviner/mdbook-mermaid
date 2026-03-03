use mdbook_mermaid::{process_sections, theme_css, themes};
use serde_json::Value;

fn main() {
    if std::env::args().nth(1).as_deref() == Some("supports") {
        std::process::exit(0);
    }

    let input: Vec<Value> =
        serde_json::from_reader(std::io::stdin()).expect("failed to parse mdbook JSON from stdin");
    let mut book = input
        .get(1)
        .expect("expected [context, book] from mdbook")
        .clone();

    let all_themes = themes();
    let css = theme_css(&all_themes);

    if let Some(sections) = book.get_mut("sections").and_then(|v| v.as_array_mut()) {
        process_sections(sections, &all_themes, &css);
    }

    serde_json::to_writer(std::io::stdout(), &book).unwrap();
}
