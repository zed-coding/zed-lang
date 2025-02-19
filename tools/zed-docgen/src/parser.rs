use anyhow::Result;
use regex::Regex;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Documentation {
    pub functions: Vec<Function>,
    pub includes: Vec<Include>,
    pub module_docs: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub doc_comments: Vec<String>,
    pub is_public: bool,
    pub source: String,
}

#[derive(Debug, Serialize)]
pub struct Include {
    pub path: String,
    pub is_std: bool,
}

pub fn parse_source(source: &str, include_private: bool) -> Result<Documentation> {
    let mut doc = Documentation {
        functions: Vec::new(),
        includes: Vec::new(),
        module_docs: Vec::new(),
    };

    // Extract all documentation comments
    let doc_comment_re = Regex::new(r"(/\*\*(?:[^*]|\*[^/])*\*/)")?;
    doc.module_docs = doc_comment_re
        .captures_iter(source)
        .map(|cap| clean_doc_comment(&cap[1]))
        .collect();

    // Parse includes
    let include_re = Regex::new(r#"@include\s+[<"]([^>"]+)[>"]"#)?;
    for cap in include_re.captures_iter(source) {
        let path = cap[1].to_string();
        let is_std = cap[0].contains('<');
        doc.includes.push(Include { path, is_std });
    }

    // Parse functions with all preceding documentation comments
    let docs_pattern = Regex::new(r"(?s)(?P<docs>(/\*\*(?:[^*]|\*[^/])*\*/\s*)*)")?;
    let function_pattern = Regex::new(r"fn\s+(?P<name>[a-zA-Z_][a-zA-Z0-9_]*)\s*\((?P<params>[^)]*)\)")?;

    for cap in docs_pattern.captures_iter(source) {
        let start = cap.get(0).unwrap().end();
        if let Some(func_cap) = function_pattern.captures(&source[start..]) {
            let name = func_cap["name"].to_string();
            let params = parse_params(&func_cap["params"]);

            // Collect all documentation comments preceding the function
            let doc_comments = doc_comment_re
                .captures_iter(&cap["docs"])
                .map(|doc_cap| clean_doc_comment(&doc_cap[1]))
                .collect::<Vec<_>>();

            // Determine if function is public (no _ prefix)
            let is_public = !name.starts_with('_');

            // Skip private functions unless explicitly included
            if !is_public && !include_private {
                continue;
            }

            // Extract function source
            let source = if let Some(src) = extract_function_source(source, &name)? {
                src
            } else {
                continue; // Skip if we can't find the source
            };

            doc.functions.push(Function {
                name,
                params,
                doc_comments,
                is_public,
                source,
            });
        }
    }

    Ok(doc)
}

fn clean_doc_comment(comment: &str) -> String {
    comment
        .trim_start_matches("/**")
        .trim_end_matches("*/")
        .lines()
        .map(|line| {
            line.trim_start_matches('*')
                .trim_start()
                .to_string()
        })
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

fn parse_params(params: &str) -> Vec<String> {
    params
        .split(',')
        .map(|p| p.trim().to_string())
        .filter(|p| !p.is_empty())
        .collect()
}

fn extract_function_source(source: &str, function_name: &str) -> Result<Option<String>> {
    let pattern = format!(
        r"fn\s+{}\s*\([^)]*\)\s*({{\s*(?:[^{{}}]|(?R))*\s*}})",
        regex::escape(function_name)
    );

    // First try with recursive pattern
    if let Ok(re) = Regex::new(&pattern) {
        if let Some(cap) = re.captures(source) {
            return Ok(Some(cap[1].trim().to_string()));
        }
    }

    // Fallback to simpler pattern that matches until the next function declaration
    let fallback_pattern = format!(
        r"fn\s+{}\s*\([^)]*\)\s*({{[\s\S]*?(?=\bfn\b|$)}})",
        regex::escape(function_name)
    );

    if let Ok(re) = Regex::new(&fallback_pattern) {
        if let Some(cap) = re.captures(source) {
            return Ok(Some(cap[1].trim().to_string()));
        }
    }

    Ok(None)
}
