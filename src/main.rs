use mermaid_rs_renderer::{LayoutConfig, RenderOptions, Theme, render_with_options};
use serde_json::Value;

fn dark_theme() -> Theme {
    Theme {
        font_family: "Inter, ui-sans-serif, system-ui, sans-serif".into(),
        font_size: 16.0,
        primary_color: "#394b70".into(),
        primary_text_color: "#c0caf5".into(),
        primary_border_color: "#565f89".into(),
        line_color: "#565f89".into(),
        secondary_color: "#24283b".into(),
        tertiary_color: "#1a1b26".into(),
        edge_label_background: "#1a1b26".into(),
        cluster_background: "#24283b".into(),
        cluster_border: "#565f89".into(),
        background: "transparent".into(),
        sequence_actor_fill: "#394b70".into(),
        sequence_actor_border: "#565f89".into(),
        sequence_actor_line: "#565f89".into(),
        sequence_note_fill: "#394b70".into(),
        sequence_note_border: "#565f89".into(),
        sequence_activation_fill: "#24283b".into(),
        sequence_activation_border: "#565f89".into(),
        text_color: "#c0caf5".into(),
        ..Theme::modern()
    }
}

fn render_mermaid(code: &str) -> String {
    let opts = RenderOptions {
        theme: dark_theme(),
        layout: LayoutConfig {
            node_spacing: 80.0,
            rank_spacing: 100.0,
            ..LayoutConfig::default()
        },
    };
    match render_with_options(code, opts) {
        Ok(svg) => svg,
        Err(e) => format!("<pre><code>mermaid error: {e}</code></pre>"),
    }
}

fn process_content(content: &str) -> String {
    let mut result = String::with_capacity(content.len());
    let mut rest = content;

    while let Some(start) = rest.find("```mermaid") {
        result.push_str(&rest[..start]);
        let after = &rest[start + "```mermaid".len()..];
        if let Some(end) = after.find("```") {
            let code = after[..end].trim();
            result.push_str(&render_mermaid(code));
            rest = &after[end + 3..];
        } else {
            result.push_str(&rest[start..]);
            return result;
        }
    }
    result.push_str(rest);
    result
}

fn process_sections(sections: &mut Vec<Value>) {
    for section in sections {
        if let Some(chapter) = section.get_mut("Chapter") {
            if let Some(content) = chapter["content"].as_str() {
                chapter["content"] = Value::String(process_content(content));
            }
            if let Some(subs) = chapter.get_mut("sub_items").and_then(|v| v.as_array_mut()) {
                process_sections(subs);
            }
        }
    }
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("supports") {
        std::process::exit(0);
    }

    let input: Vec<Value> = serde_json::from_reader(std::io::stdin()).unwrap();
    let mut book = input[1].clone();

    if let Some(sections) = book.get_mut("sections").and_then(|v| v.as_array_mut()) {
        process_sections(sections);
    }

    serde_json::to_writer(std::io::stdout(), &book).unwrap();
}
