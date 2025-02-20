use anyhow::Result;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    pub indent_spaces: usize,
    pub max_width: usize,
}

pub fn format_source(source: &str, config: &Config) -> Result<String> {
    let mut formatter = Formatter::new(config);
    formatter.format(source)
}

#[allow(dead_code)]
struct Formatter<'a> {
    config: &'a Config,
    output: String,
    indent_level: usize,
}

impl<'a> Formatter<'a> {
    fn new(config: &'a Config) -> Self {
        Self {
            config,
            output: String::new(),
            indent_level: 0,
        }
    }

    fn format(&mut self, source: &str) -> Result<String> {
        // Normalize line endings
        let source = source.replace("\r\n", "\n");

        // Process each line
        for line in source.lines() {
            self.format_line(line)?;
        }

        // Ensure final newline
        if !self.output.ends_with('\n') {
            self.output.push('\n');
        }

        Ok(self.output.clone())
    }

    fn format_line(&mut self, line: &str) -> Result<()> {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            self.output.push('\n');
            return Ok(());
        }

        // Handle braces and control structures for indentation
        if trimmed.starts_with('}') {
            self.indent_level = self.indent_level.saturating_sub(1);
        }

        // Write the line with proper indentation
        self.write_indented(trimmed)?;

        // Adjust indentation for next line - handles both braces and control structures
        if trimmed.ends_with('{') ||
           (trimmed.starts_with("if") && !trimmed.ends_with('}')) ||
           (trimmed.starts_with("while") && !trimmed.ends_with('}')) {
            self.indent_level += 1;
        }

        Ok(())
    }

    fn write_indented(&mut self, content: &str) -> Result<()> {
        let indent = " ".repeat(self.indent_level * self.config.indent_spaces);
        self.output.push_str(&indent);
        self.output.push_str(content);
        self.output.push('\n');
        Ok(())
    }
}
