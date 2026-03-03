use mermaid_rs_renderer::{LayoutConfig, RenderOptions, Theme, render_with_options};
use serde_json::Value;

struct ThemeVariant {
    name: &'static str,
    theme: Theme,
}

fn base(overrides: Theme) -> Theme {
    Theme {
        font_family: "Inter, ui-sans-serif, system-ui, sans-serif".into(),
        font_size: 16.0,
        background: "transparent".into(),
        ..overrides
    }
}

fn themes() -> Vec<ThemeVariant> {
    vec![
        ThemeVariant {
            name: "light",
            theme: base(Theme::modern()),
        },
        ThemeVariant {
            name: "navy",
            theme: base(Theme {
                primary_color: "#394b70".into(),
                primary_text_color: "#c0caf5".into(),
                primary_border_color: "#565f89".into(),
                line_color: "#565f89".into(),
                secondary_color: "#24283b".into(),
                tertiary_color: "#1a1b26".into(),
                edge_label_background: "#1a1b26".into(),
                cluster_background: "#24283b".into(),
                cluster_border: "#565f89".into(),
                sequence_actor_fill: "#394b70".into(),
                sequence_actor_border: "#565f89".into(),
                sequence_actor_line: "#565f89".into(),
                sequence_note_fill: "#394b70".into(),
                sequence_note_border: "#565f89".into(),
                sequence_activation_fill: "#24283b".into(),
                sequence_activation_border: "#565f89".into(),
                text_color: "#c0caf5".into(),
                ..Theme::modern()
            }),
        },
        ThemeVariant {
            name: "coal",
            theme: base(Theme {
                primary_color: "#3a3a3a".into(),
                primary_text_color: "#e0e0e0".into(),
                primary_border_color: "#555".into(),
                line_color: "#555".into(),
                secondary_color: "#2a2a2a".into(),
                tertiary_color: "#1a1a1a".into(),
                edge_label_background: "#1a1a1a".into(),
                cluster_background: "#2a2a2a".into(),
                cluster_border: "#555".into(),
                sequence_actor_fill: "#3a3a3a".into(),
                sequence_actor_border: "#555".into(),
                sequence_actor_line: "#555".into(),
                sequence_note_fill: "#3a3a3a".into(),
                sequence_note_border: "#555".into(),
                sequence_activation_fill: "#2a2a2a".into(),
                sequence_activation_border: "#555".into(),
                text_color: "#e0e0e0".into(),
                ..Theme::modern()
            }),
        },
        ThemeVariant {
            name: "ayu",
            theme: base(Theme {
                primary_color: "#2d3440".into(),
                primary_text_color: "#bfbdb6".into(),
                primary_border_color: "#444b55".into(),
                line_color: "#444b55".into(),
                secondary_color: "#1a1e28".into(),
                tertiary_color: "#0b0e14".into(),
                edge_label_background: "#0b0e14".into(),
                cluster_background: "#1a1e28".into(),
                cluster_border: "#444b55".into(),
                sequence_actor_fill: "#2d3440".into(),
                sequence_actor_border: "#444b55".into(),
                sequence_actor_line: "#444b55".into(),
                sequence_note_fill: "#2d3440".into(),
                sequence_note_border: "#444b55".into(),
                sequence_activation_fill: "#1a1e28".into(),
                sequence_activation_border: "#444b55".into(),
                text_color: "#bfbdb6".into(),
                ..Theme::modern()
            }),
        },
        ThemeVariant {
            name: "rust",
            theme: base(Theme {
                primary_color: "#d4b8a0".into(),
                primary_text_color: "#3b2e2a".into(),
                primary_border_color: "#8b6c54".into(),
                line_color: "#8b6c54".into(),
                secondary_color: "#e8d5c4".into(),
                tertiary_color: "#f0e4d6".into(),
                edge_label_background: "transparent".into(),
                cluster_background: "#e8d5c4".into(),
                cluster_border: "#8b6c54".into(),
                sequence_actor_fill: "#d4b8a0".into(),
                sequence_actor_border: "#8b6c54".into(),
                sequence_actor_line: "#8b6c54".into(),
                sequence_note_fill: "#e8d5c4".into(),
                sequence_note_border: "#8b6c54".into(),
                sequence_activation_fill: "#e8d5c4".into(),
                sequence_activation_border: "#8b6c54".into(),
                text_color: "#3b2e2a".into(),
                ..Theme::modern()
            }),
        },
    ]
}

fn layout() -> LayoutConfig {
    LayoutConfig {
        node_spacing: 80.0,
        rank_spacing: 100.0,
        ..LayoutConfig::default()
    }
}

fn render_svg(code: &str, theme: Theme) -> String {
    let opts = RenderOptions {
        theme,
        layout: layout(),
    };
    match render_with_options(code, opts) {
        Ok(svg) => svg,
        Err(e) => format!("<pre><code>mermaid error: {e}</code></pre>"),
    }
}

fn theme_css() -> String {
    let names: Vec<&str> = themes().iter().map(|t| t.name).collect();
    let mut css = String::from("<style>");
    for name in &names {
        // Hide this variant on every other theme
        for other in &names {
            if other != name {
                css.push_str(&format!(".{other} .mermaid-{name},"));
            }
        }
    }
    // Remove trailing comma and close
    css.pop();
    css.push_str(" { display: none; }");
    css.push_str("</style>\n");
    css
}

fn render_mermaid(code: &str, all_themes: &[ThemeVariant]) -> String {
    let mut out = String::new();
    for variant in all_themes {
        let svg = render_svg(code, variant.theme.clone());
        out.push_str(&format!(
            "<div class=\"mermaid-{}\">{svg}</div>",
            variant.name
        ));
    }
    out
}

fn process_content(content: &str, all_themes: &[ThemeVariant], css: &str) -> String {
    let mut result = String::with_capacity(content.len());
    let mut rest = content;
    let mut css_injected = false;

    while let Some(start) = rest.find("```mermaid") {
        if !css_injected {
            result.push_str(css);
            css_injected = true;
        }
        result.push_str(&rest[..start]);
        let after = &rest[start + "```mermaid".len()..];
        if let Some(end) = after.find("```") {
            let code = after[..end].trim();
            result.push_str(&render_mermaid(code, all_themes));
            rest = &after[end + 3..];
        } else {
            result.push_str(&rest[start..]);
            return result;
        }
    }
    result.push_str(rest);
    result
}

fn process_sections(sections: &mut Vec<Value>, all_themes: &[ThemeVariant], css: &str) {
    for section in sections {
        if let Some(chapter) = section.get_mut("Chapter") {
            if let Some(content) = chapter["content"].as_str() {
                chapter["content"] =
                    Value::String(process_content(content, all_themes, css));
            }
            if let Some(subs) = chapter.get_mut("sub_items").and_then(|v| v.as_array_mut()) {
                process_sections(subs, all_themes, css);
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

    let all_themes = themes();
    let css = theme_css();

    if let Some(sections) = book.get_mut("sections").and_then(|v| v.as_array_mut()) {
        process_sections(sections, &all_themes, &css);
    }

    serde_json::to_writer(std::io::stdout(), &book).unwrap();
}
