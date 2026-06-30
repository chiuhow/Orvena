//! Per-step parsing: the model speaks a tiny action protocol so the loop can
//! apply changes deterministically. v0.1 supports a single action — writing a
//! file in full:
//!
//! ```text
//! <<<WRITE relative/path
//! <full new file content>
//! >>>
//! ```

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Write { path: String, content: String },
}

/// Parse zero or more `<<<WRITE …>>>` blocks from a model response.
pub fn parse_actions(text: &str) -> Vec<Action> {
    let mut actions = Vec::new();
    let mut lines = text.lines();
    while let Some(line) = lines.next() {
        let trimmed = line.trim_start();
        if let Some(path) = trimmed.strip_prefix("<<<WRITE ") {
            let path = path.trim().to_string();
            let mut content_lines = Vec::new();
            for body in lines.by_ref() {
                if body.trim() == ">>>" {
                    break;
                }
                content_lines.push(body);
            }
            let mut content = content_lines.join("\n");
            if !content.is_empty() {
                content.push('\n');
            }
            actions.push(Action::Write { path, content });
        }
    }
    actions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_single_write() {
        let text = "preamble\n<<<WRITE src/a.txt\nhello\nworld\n>>>\ntrailer";
        let actions = parse_actions(text);
        assert_eq!(
            actions,
            vec![Action::Write {
                path: "src/a.txt".into(),
                content: "hello\nworld\n".into()
            }]
        );
    }

    #[test]
    fn parses_none_when_absent() {
        assert!(parse_actions("just prose, no actions").is_empty());
    }
}
