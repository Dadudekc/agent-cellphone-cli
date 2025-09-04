pub struct Edit {
    pub line: usize,
    pub content: String,
}

impl Edit {
    pub fn apply(&self, text: &str) -> String {
        let mut lines: Vec<_> = text.lines().collect();
        if self.line < lines.len() {
            lines[self.line] = &self.content;
        }
        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_apply() {
        let edit = Edit { line: 0, content: "hi".into() };
        assert_eq!(edit.apply("bye"), "hi");
    }
}
