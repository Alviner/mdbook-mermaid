use mermaid_rs_renderer::{LayoutConfig, RenderOptions, Theme, render_with_options};
use serde_json::Value;

pub struct ThemeVariant {
    pub name: &'static str,
    pub theme: Theme,
}

fn base(overrides: Theme) -> Theme {
    Theme {
        font_family: "Inter, ui-sans-serif, system-ui, sans-serif".into(),
        font_size: 16.0,
        background: "transparent".into(),
        ..overrides
    }
}

pub fn themes() -> Vec<ThemeVariant> {
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
        Err(e) => {
            let msg = e
                .to_string()
                .replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;");
            format!("<pre><code>mermaid error: {msg}</code></pre>")
        }
    }
}

pub fn theme_css(all_themes: &[ThemeVariant]) -> String {
    let names: Vec<&str> = all_themes.iter().map(|t| t.name).collect();
    let selectors: Vec<String> = names
        .iter()
        .flat_map(|name| {
            names
                .iter()
                .filter(move |other| *other != name)
                .map(move |other| format!(".{other} .mermaid-{name}"))
        })
        .collect();
    format!(
        "<style>{} {{ display: none; }}</style>\n",
        selectors.join(",")
    )
}

pub fn render_mermaid(code: &str, all_themes: &[ThemeVariant]) -> String {
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

pub fn process_content(content: &str, all_themes: &[ThemeVariant], css: &str) -> String {
    let mut result = String::with_capacity(content.len());
    let mut rest = content;
    let mut css_injected = false;

    while let Some(start) = rest.find("```mermaid\n") {
        if !css_injected {
            result.push_str(css);
            css_injected = true;
        }
        result.push_str(&rest[..start]);
        let after = &rest[start + "```mermaid\n".len()..];
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

pub fn process_sections(sections: &mut [Value], all_themes: &[ThemeVariant], css: &str) {
    for section in sections {
        if let Some(chapter) = section.get_mut("Chapter") {
            if let Some(content) = chapter["content"].as_str() {
                chapter["content"] = Value::String(process_content(content, all_themes, css));
            }
            if let Some(subs) = chapter.get_mut("sub_items").and_then(|v| v.as_array_mut()) {
                process_sections(subs, all_themes, css);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_content_no_mermaid() {
        let all_themes = themes();
        let css = theme_css(&all_themes);
        let input = "# Hello\n\nSome text without mermaid blocks.";
        let result = process_content(input, &all_themes, &css);
        assert_eq!(result, input);
    }

    #[test]
    fn process_content_with_mermaid_block() {
        let all_themes = themes();
        let css = theme_css(&all_themes);
        let input = "before\n```mermaid\ngraph LR\n  A --> B\n```\nafter";
        let result = process_content(input, &all_themes, &css);

        assert!(result.starts_with("<style>"));
        assert!(result.contains("before\n"));
        assert!(result.contains("after"));
        assert!(!result.contains("```mermaid"));
        for variant in &all_themes {
            assert!(result.contains(&format!("class=\"mermaid-{}\"", variant.name)));
        }
    }

    #[test]
    fn process_content_css_injected_once() {
        let all_themes = themes();
        let css = theme_css(&all_themes);
        let input = "```mermaid\ngraph LR\n  A --> B\n```\n```mermaid\ngraph LR\n  C --> D\n```";
        let result = process_content(input, &all_themes, &css);
        assert_eq!(result.matches("<style>").count(), 1);
    }

    #[test]
    fn process_content_unclosed_block() {
        let all_themes = themes();
        let css = theme_css(&all_themes);
        let input = "before\n```mermaid\ngraph LR\n  A --> B";
        let result = process_content(input, &all_themes, &css);
        assert!(result.contains("```mermaid"));
    }

    #[test]
    fn process_content_multiple_blocks() {
        let all_themes = themes();
        let css = theme_css(&all_themes);
        let input =
            "```mermaid\ngraph LR\n  A --> B\n```\nmiddle\n```mermaid\ngraph LR\n  C --> D\n```";
        let result = process_content(input, &all_themes, &css);

        let div_count = result.matches("<div class=\"mermaid-light\">").count();
        assert_eq!(div_count, 2);
    }

    #[test]
    fn theme_css_generates_valid_css() {
        let all_themes = themes();
        let css = theme_css(&all_themes);
        assert!(css.starts_with("<style>"));
        assert!(css.contains("display: none;"));
        assert!(css.ends_with("</style>\n"));

        let names: Vec<&str> = themes().iter().map(|t| t.name).collect();
        for name in &names {
            assert!(css.contains(&format!(".mermaid-{name}")));
        }
    }

    #[test]
    fn render_mermaid_creates_div_per_theme() {
        let all_themes = themes();
        let result = render_mermaid("graph LR\n  A --> B", &all_themes);

        for variant in &all_themes {
            assert!(result.contains(&format!("<div class=\"mermaid-{}\">", variant.name)));
        }
        assert_eq!(result.matches("<div ").count(), all_themes.len());
    }

    #[test]
    fn process_sections_handles_chapters() {
        let all_themes = themes();
        let css = theme_css(&all_themes);
        let mut sections: Vec<Value> = serde_json::from_str(
            r#"[
            {
                "Chapter": {
                    "name": "Test",
                    "content": "```mermaid\ngraph LR\n  A --> B\n```",
                    "sub_items": []
                }
            }
        ]"#,
        )
        .unwrap();

        process_sections(&mut sections, &all_themes, &css);

        let content = sections[0]["Chapter"]["content"].as_str().unwrap();
        assert!(!content.contains("```mermaid"));
        assert!(content.contains("mermaid-light"));
    }

    #[test]
    fn process_sections_handles_nested_chapters() {
        let all_themes = themes();
        let css = theme_css(&all_themes);
        let mut sections: Vec<Value> = serde_json::from_str(
            r#"[
            {
                "Chapter": {
                    "name": "Parent",
                    "content": "no mermaid here",
                    "sub_items": [
                        {
                            "Chapter": {
                                "name": "Child",
                                "content": "```mermaid\ngraph LR\n  X --> Y\n```",
                                "sub_items": []
                            }
                        }
                    ]
                }
            }
        ]"#,
        )
        .unwrap();

        process_sections(&mut sections, &all_themes, &css);

        let parent_content = sections[0]["Chapter"]["content"].as_str().unwrap();
        assert_eq!(parent_content, "no mermaid here");

        let child_content = sections[0]["Chapter"]["sub_items"][0]["Chapter"]["content"]
            .as_str()
            .unwrap();
        assert!(child_content.contains("mermaid-light"));
    }

    #[test]
    fn process_content_ignores_non_mermaid_fenced_blocks() {
        let all_themes = themes();
        let css = theme_css(&all_themes);
        let input = "```mermaid-test\nsome content\n```";
        let result = process_content(input, &all_themes, &css);
        assert_eq!(result, input);
    }

    #[test]
    fn render_svg_escapes_html_in_errors() {
        let result = render_svg("<script>alert(1)</script>", Theme::modern());
        assert!(result.contains("&lt;script&gt;"));
        assert!(!result.contains("<script>"));
    }
}
