use anyhow::Result;
use regex::Regex;

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
    in_comment: bool,
    in_string: bool,
    in_asm: bool,
}

impl<'a> Formatter<'a> {
    fn new(config: &'a Config) -> Self {
        Self {
            config,
            output: String::new(),
            indent_level: 0,
            in_comment: false,
            in_string: false,
            in_asm: false,
        }
    }

    fn format(&mut self, source: &str) -> Result<String> {
        // First, normalize line endings
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

        // Handle block comments
        if trimmed.starts_with("/*") {
            self.in_comment = true;
        }
        if self.in_comment {
            self.write_indented(line)?;
            if trimmed.ends_with("*/") {
                self.in_comment = false;
            }
            return Ok(());
        }

        // Handle inline assembly
        if trimmed.starts_with("asm") {
            self.in_asm = true;
        }
        if self.in_asm {
            self.write_indented(line)?;
            if line.contains(";") && !line.trim_start().starts_with("//") {
                self.in_asm = false;
            }
            return Ok(());
        }

        // Adjust indentation based on braces
        if trimmed.starts_with('}') {
            self.indent_level = self.indent_level.saturating_sub(1);
        }

        // Format the line
        let formatted = self.format_code_line(line)?;
        self.write_indented(&formatted)?;

        // Adjust indentation for next line
        if trimmed.ends_with('{') {
            self.indent_level += 1;
        }

        Ok(())
    }

    fn format_code_line(&mut self, line: &str) -> Result<String> {
        let mut result = String::new();
        let trimmed = line.trim();

        // Handle special cases
        if trimmed.starts_with("//") {
            return Ok(line.to_string());
        }

        // Format operators with spaces
        let operators = Regex::new(r"([+\-*/=<>!]=?|&&|\|\|)")?;
        let with_spaces = operators.replace_all(trimmed, " $1 ");

        // Remove extra spaces
        let mut parts: Vec<_> = with_spaces.split_whitespace().collect();

        // Handle semicolons
        if let Some(last) = parts.last_mut() {
            if last.ends_with(';') {
                *last = &last[..last.len() - 1];
                parts.push(";");
            }
        }

        result.push_str(&parts.join(" "));

        Ok(result)
    }

    fn write_indented(&mut self, content: &str) -> Result<()> {
        let indent = " ".repeat(self.indent_level * self.config.indent_spaces);
        self.output.push_str(&indent);
        self.output.push_str(content);
        self.output.push('\n');
        Ok(())
    }
}
