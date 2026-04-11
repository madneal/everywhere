use std::path::Path;

use anyhow::Result;
use pulldown_cmark::{Event, Options, Parser};
use tokio::fs;

fn strip_front_matter(s: &str) -> &str {
    let s = s.strip_prefix('\u{feff}').unwrap_or(s);
    let rest = if let Some(r) = s.strip_prefix("---\r\n") {
        r
    } else if let Some(r) = s.strip_prefix("---\n") {
        r
    } else {
        return s;
    };
    if let Some(body) = rest.strip_prefix("---\r\n").or_else(|| rest.strip_prefix("---\n")) {
        return body;
    }
    let mut pos = 0usize;
    let mut line_index = 0usize;
    while pos < rest.len() {
        let slice = &rest[pos..];
        let (end_rel, advance) = if let Some(i) = slice.find('\n') {
            (i, i + 1)
        } else {
            (slice.len(), slice.len())
        };
        let line = slice.get(..end_rel).unwrap_or("").trim_end_matches('\r');
        let trimmed = line.trim();
        if line_index >= 1 && trimmed == "---" {
            let body_start = (pos + advance).min(rest.len());
            return rest.get(body_start..).unwrap_or("");
        }
        pos += advance;
        line_index += 1;
    }
    s
}

fn markdown_to_plain(src: &str) -> String {
    let mut out = String::new();
    let parser = Parser::new_ext(
        src,
        Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TASKLISTS,
    );
    for event in parser {
        match event {
            Event::Text(t) | Event::Code(t) => {
                let need_space = !out.is_empty()
                    && !out
                        .chars()
                        .last()
                        .map_or(true, |c| c.is_whitespace());
                if need_space {
                    out.push(' ');
                }
                out.push_str(&t);
            }
            Event::SoftBreak | Event::HardBreak => out.push(' '),
            _ => {}
        }
    }
    out
}

pub async fn extract_text(path: &Path) -> Result<String> {
    let raw = match fs::read_to_string(path).await {
        Ok(s) => s,
        Err(_) => {
            let b = fs::read(path).await?;
            String::from_utf8_lossy(&b).into_owned()
        }
    };
    let stripped = strip_front_matter(&raw);
    let mut plain = markdown_to_plain(stripped);
    if plain.trim().is_empty() && !raw.trim().is_empty() {
        plain = markdown_to_plain(&raw);
    }
    if plain.trim().is_empty() && !raw.trim().is_empty() {
        plain = raw.clone();
    }
    Ok(plain)
}
