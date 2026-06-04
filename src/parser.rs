use std::path::Path;

use crate::model::{CodeItem, ItemKind};

pub fn parse_code_items(content: &str, file: &Path) -> Vec<CodeItem> {
    let mut items = Vec::new();
    let mut in_block_comment = false;

    for (index, line) in content.lines().enumerate() {
        let line_number = index + 1;
        let searchable = strip_comments(line, &mut in_block_comment);
        let trimmed = searchable.trim_start();

        if trimmed.is_empty() {
            continue;
        }

        if let Some(name) = extract_ident_after_keyword(trimmed, "fn") {
            items.push(CodeItem {
                name,
                kind: ItemKind::Function,
                file: file.to_path_buf(),
                line: line_number,
            });
        }

        if let Some(name) = extract_ident_after_keyword(trimmed, "struct") {
            items.push(CodeItem {
                name,
                kind: ItemKind::Struct,
                file: file.to_path_buf(),
                line: line_number,
            });
        }

        if let Some(name) = extract_ident_after_keyword(trimmed, "enum") {
            items.push(CodeItem {
                name,
                kind: ItemKind::Enum,
                file: file.to_path_buf(),
                line: line_number,
            });
        }

        if let Some(name) = extract_ident_after_keyword(trimmed, "trait") {
            items.push(CodeItem {
                name,
                kind: ItemKind::Trait,
                file: file.to_path_buf(),
                line: line_number,
            });
        }

        if let Some(name) = extract_ident_after_keyword(trimmed, "mod") {
            items.push(CodeItem {
                name,
                kind: ItemKind::Module,
                file: file.to_path_buf(),
                line: line_number,
            });
        }

        if let Some(name) = extract_impl_name(trimmed) {
            items.push(CodeItem {
                name,
                kind: ItemKind::Impl,
                file: file.to_path_buf(),
                line: line_number,
            });
        }
    }

    items
}

pub(crate) fn strip_comments(line: &str, in_block_comment: &mut bool) -> String {
    let mut output = String::new();
    let mut chars = line.chars().peekable();

    while let Some(current) = chars.next() {
        if *in_block_comment {
            if current == '*' && chars.peek() == Some(&'/') {
                chars.next();
                *in_block_comment = false;
            }
            continue;
        }

        if current == '/' {
            match chars.peek() {
                Some('/') => break,
                Some('*') => {
                    chars.next();
                    *in_block_comment = true;
                    continue;
                }
                _ => {}
            }
        }

        output.push(current);
    }

    output
}

pub(crate) fn extract_ident_after_keyword(line: &str, keyword: &str) -> Option<String> {
    let position = find_keyword(line, keyword)?;
    let after_keyword = &line[position + keyword.len()..];
    let candidate = after_keyword.trim_start();
    let name: String = candidate
        .chars()
        .take_while(|character| character.is_ascii_alphanumeric() || *character == '_')
        .collect();

    if name.is_empty() { None } else { Some(name) }
}

pub(crate) fn find_keyword(line: &str, keyword: &str) -> Option<usize> {
    line.match_indices(keyword)
        .find(|(position, _)| has_keyword_boundaries(line, *position, keyword.len()))
        .map(|(position, _)| position)
}

fn has_keyword_boundaries(line: &str, position: usize, len: usize) -> bool {
    let before = if position == 0 {
        None
    } else {
        line[..position].chars().next_back()
    };
    let after = line[position + len..].chars().next();

    !before.is_some_and(is_ident_char) && !after.is_some_and(is_ident_char)
}

fn is_ident_char(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_'
}

fn extract_impl_name(line: &str) -> Option<String> {
    let position = find_keyword(line, "impl")?;
    let after_keyword = line[position + "impl".len()..].trim_start();

    if after_keyword.is_empty() {
        return None;
    }

    let before_body = after_keyword
        .split(['{', ';'])
        .next()
        .unwrap_or_default()
        .split(" where ")
        .next()
        .unwrap_or_default()
        .trim();

    if before_body.is_empty() {
        Some("<anonymous impl>".to_string())
    } else {
        Some(before_body.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn recognizes_basic_rust_items_and_skips_comments() {
        let file = PathBuf::from("src/lib.rs");
        let content = r#"
// fn ignored() {}
pub mod analyzer;
pub(crate) fn analyze_file() {}
struct Report;
enum Status { Ready }
trait Render { fn render(&self); }
impl Report { pub fn new() -> Self { Self } }
"#;

        let items = parse_code_items(content, &file);

        assert!(
            items
                .iter()
                .any(|item| item.kind == ItemKind::Module && item.name == "analyzer")
        );
        assert!(
            items
                .iter()
                .any(|item| item.kind == ItemKind::Function && item.name == "analyze_file")
        );
        assert!(
            items
                .iter()
                .any(|item| item.kind == ItemKind::Struct && item.name == "Report")
        );
        assert!(
            items
                .iter()
                .any(|item| item.kind == ItemKind::Enum && item.name == "Status")
        );
        assert!(
            items
                .iter()
                .any(|item| item.kind == ItemKind::Trait && item.name == "Render")
        );
        assert!(
            items
                .iter()
                .any(|item| item.kind == ItemKind::Impl && item.name == "Report")
        );
        assert!(!items.iter().any(|item| item.name == "ignored"));
    }
}
