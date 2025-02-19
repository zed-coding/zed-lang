use anyhow::Result;
use pulldown_cmark::{html, Parser};
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use tera::{Context, Tera};
use std::fs;
use std::path::Path;
use crate::parser::Documentation;
use crate::templates::{DOC_TEMPLATE, INDEX_TEMPLATE};

pub const STYLE_CSS: &str = r#"/* Zed Documentation Style */
:root {
    --bg-color: #1a1a1a;
    --text-color: #e0e0e0;
    --link-color: #4a9eff;
    --border-color: #333;
    --code-bg: #2d2d2d;
    --header-bg: #252525;
    --function-bg: #202020;
    --highlight-color: #3c3c3c;
    --public-color: #4aff9e;
    --private-color: #ff4a4a;
    --std-include-color: #4a9eff;
    --local-include-color: #4aff9e;
}

body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    line-height: 1.6;
    color: var(--text-color);
    background: var(--bg-color);
    margin: 0;
    padding: 0;
}

.container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
}

header {
    background: var(--header-bg);
    padding: 1rem 0;
    margin-bottom: 2rem;
    border-bottom: 1px solid var(--border-color);
    position: sticky;
    top: 0;
    z-index: 100;
}

header .container {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 2rem;
}

nav {
    display: flex;
    gap: 1rem;
    align-items: center;
}

.nav-separator {
    color: var(--border-color);
}

h1, h2, h3, h4 {
    color: var(--text-color);
    margin-top: 2rem;
    margin-bottom: 1rem;
}

a {
    color: var(--link-color);
    text-decoration: none;
}

a:hover {
    text-decoration: underline;
}

section {
    margin: 3rem 0;
    padding-top: 2rem;
    border-top: 1px solid var(--border-color);
}

/* Module Documentation */
.module-doc {
    background: var(--function-bg);
    padding: 1.5rem;
    border-radius: 4px;
    margin-bottom: 2rem;
    border: 1px solid var(--border-color);
}

/* Includes Section */
.includes {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 2rem;
    margin-bottom: 2rem;
}

.include-item {
    background: var(--highlight-color);
    padding: 0.5rem 1rem;
    border-radius: 4px;
    margin-bottom: 0.5rem;
}

.std-include code {
    color: var(--std-include-color);
}

.local-include code {
    color: var(--local-include-color);
}

/* Function List */
.function-list {
    background: var(--function-bg);
    padding: 1.5rem;
    border-radius: 4px;
    margin-bottom: 2rem;
}

.function-list ul {
    list-style: none;
    padding: 0;
    margin: 0;
}

.function-list li {
    display: flex;
    align-items: center;
    padding: 0.5rem 0;
    border-bottom: 1px solid var(--border-color);
}

.function-list li:last-child {
    border-bottom: none;
}

.visibility-badge {
    margin-left: 1rem;
    padding: 0.2rem 0.6rem;
    border-radius: 3px;
    font-size: 0.8rem;
}

.visibility-badge.public {
    background: color-mix(in srgb, var(--public-color) 20%, transparent);
    color: var(--public-color);
}

.visibility-badge.private {
    background: color-mix(in srgb, var(--private-color) 20%, transparent);
    color: var(--private-color);
}

/* Function Documentation */
.function {
    background: var(--function-bg);
    padding: 1.5rem;
    border-radius: 4px;
    margin-bottom: 1.5rem;
    border: 1px solid var(--border-color);
    transition: transform 0.2s;
}

.function:hover {
    transform: translateY(-2px);
}

.function-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
    padding-bottom: 1rem;
    border-bottom: 1px solid var(--border-color);
}

.function-signature {
    font-family: 'Fira Code', 'Cascadia Code', monospace;
}

.fn-keyword {
    color: #c586c0;
}

.function-name {
    color: #4a9eff;
}

.function-params {
    color: #888;
}

.function-visibility {
    padding: 0.2rem 0.6rem;
    border-radius: 3px;
    font-size: 0.9rem;
}

.function-doc {
    margin: 1.5rem 0;
    line-height: 1.6;
}

.function-doc h1,
.function-doc h2,
.function-doc h3 {
    color: var(--text-color);
    margin: 1.5rem 0 1rem;
}

.function-doc p {
    margin: 1rem 0;
}

.function-doc code {
    background: var(--code-bg);
    padding: 0.2rem 0.4rem;
    border-radius: 3px;
    font-family: 'Fira Code', 'Cascadia Code', monospace;
}

.function-source {
    background: var(--code-bg);
    border-radius: 4px;
    overflow: hidden;
    margin-top: 1.5rem;
}

.source-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1rem;
    background: color-mix(in srgb, var(--code-bg) 50%, black);
    border-bottom: 1px solid var(--border-color);
}

.toggle-source {
    background: var(--highlight-color);
    border: 1px solid var(--border-color);
    color: var(--text-color);
    padding: 0.25rem 0.75rem;
    border-radius: 3px;
    cursor: pointer;
    transition: background-color 0.2s;
}

.toggle-source:hover {
    background: color-mix(in srgb, var(--highlight-color) 80%, white);
}

.source-code {
    padding: 1rem;
    overflow-x: auto;
    font-family: 'Fira Code', 'Cascadia Code', monospace;
}

.back-to-top {
    display: inline-block;
    margin-top: 1rem;
    padding: 0.5rem 1rem;
    background: var(--highlight-color);
    border-radius: 4px;
    font-size: 0.9rem;
    transition: background-color 0.2s;
}

.back-to-top:hover {
    background: color-mix(in srgb, var(--highlight-color) 80%, white);
    text-decoration: none;
}

/* Search Box */
.search-box {
    margin-bottom: 1.5rem;
}

.search-box input {
    width: 100%;
    padding: 0.75rem 1rem;
    background: var(--code-bg);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    color: var(--text-color);
    font-size: 1rem;
}

.search-box input:focus {
    outline: none;
    border-color: var(--link-color);
}

/* File List */
.file-list {
    list-style: none;
    padding: 0;
}

.file-item {
    background: var(--function-bg);
    margin-bottom: 0.5rem;
    padding: 1rem;
    border-radius: 4px;
    border: 1px solid var(--border-color);
    display: flex;
    justify-content: space-between;
    align-items: center;
    transition: transform 0.2s;
}

.file-item:hover {
    transform: translateY(-2px);
}

.file-type {
    font-size: 0.8rem;
    padding: 0.2rem 0.6rem;
    background: var(--highlight-color);
    border-radius: 3px;
    color: #888;
}

/* Help Section */
.index-help {
    margin-top: 3rem;
    padding-top: 2rem;
    border-top: 1px solid var(--border-color);
}

.index-help ul {
    padding-left: 1.5rem;
}

.index-help li {
    margin: 0.5rem 0;
}

/* Footer */
footer {
    margin-top: 4rem;
    padding: 2rem 0;
    background: var(--header-bg);
    border-top: 1px solid var(--border-color);
    text-align: center;
    color: #888;
}

/* Responsive Design */
@media (max-width: 768px) {
    .container {
        padding: 1rem;
    }

    header .container {
        flex-direction: column;
        text-align: center;
        gap: 1rem;
    }

    .function-header {
        flex-direction: column;
        align-items: flex-start;
        gap: 1rem;
    }

    .includes {
        grid-template-columns: 1fr;
    }

    .function-list li {
        flex-direction: column;
        align-items: flex-start;
        gap: 0.5rem;
    }

    .visibility-badge {
        margin-left: 0;
    }
}

/* Print Styles */
@media print {
    body {
        background: white;
        color: black;
    }

    .container {
        max-width: none;
        padding: 0;
    }

    header {
        position: static;
    }

    .toggle-source,
    .back-to-top,
    .search-box {
        display: none;
    }

    .source-code {
        overflow: visible;
        break-inside: avoid;
    }

    a {
        text-decoration: none;
        color: black;
    }
}"#;

pub fn generate_html(doc: &Documentation, title: Option<&str>) -> Result<String> {
    let mut tera = Tera::default();
    tera.add_raw_template("doc", DOC_TEMPLATE)?;

    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    // Default to Rust syntax highlighting if Zed isn't available
    let syntax = ss.find_syntax_by_extension("zed")
        .or_else(|| ss.find_syntax_by_extension("rust"))
        .or_else(|| ss.find_syntax_by_extension("c"))
        .unwrap_or_else(|| ss.find_syntax_plain_text());

    let theme = &ts.themes["base16-ocean.dark"];

    let mut context = Context::new();
    context.insert("title", &title.unwrap_or("Zed Documentation"));
    context.insert("doc", &doc);

    // Convert module documentation comments to HTML
    let module_docs_html: Vec<String> = doc.module_docs
        .iter()
        .map(|doc| markdown_to_html(doc))
        .collect();
    context.insert("module_docs_html", &module_docs_html);

    // Process function documentation
    let functions: Vec<_> = doc.functions
        .iter()
        .map(|f| {
            let mut func = serde_json::to_value(f).unwrap();

            // Convert multiple documentation comments to HTML
            let doc_html: Vec<String> = f.doc_comments
                .iter()
                .map(|doc| markdown_to_html(doc))
                .collect();
            func["doc_html"] = serde_json::Value::Array(
                doc_html.into_iter()
                    .map(|html| serde_json::Value::String(html))
                    .collect()
            );

            // Syntax highlight the source code
            if !f.source.is_empty() {
                let highlighted = highlighted_html_for_string(
                    &f.source,
                    &ss,
                    &syntax,
                    theme
                ).unwrap_or_else(|_| html_escape(&f.source));
                func["source_html"] = serde_json::Value::String(highlighted);
            }
            func
        })
        .collect();
    context.insert("functions", &functions);

    Ok(tera.render("doc", &context)?)
}

pub fn generate_index(output_dir: &Path, title: Option<&str>) -> Result<String> {
    let mut tera = Tera::default();
    tera.add_raw_template("index", INDEX_TEMPLATE)?;

    // Create assets directory and write CSS
    let assets_dir = output_dir.join("assets");
    fs::create_dir_all(&assets_dir)?;
    fs::write(assets_dir.join("style.css"), STYLE_CSS)?;

    let mut context = Context::new();
    context.insert("title", &title.unwrap_or("Zed Documentation"));

    // Collect all generated HTML files
    let mut files = Vec::new();
    for entry in fs::read_dir(output_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "html") {
            if let Some(name) = path.file_stem()
                .and_then(|s| s.to_str())
                .map(|s| s.to_string())
            {
                if name != "index" {
                    files.push(name);
                }
            }
        }
    }
    files.sort();
    context.insert("files", &files);

    let html = tera.render("index", &context)?;
    fs::write(output_dir.join("index.html"), &html)?;

    Ok(html)
}

fn markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
