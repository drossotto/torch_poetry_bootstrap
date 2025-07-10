// src/tomlgen.rs

use crate::resolver::TorchSource;

pub fn generate_poetry_source_toml(source: &TorchSource) -> String {
    format!(
        r#"
[[tool.poetry.source]]
name = "{name}"
url = "{url}"
priority = "explicit"
"#,
        name = source.source,
        url = source.url
    )
}