use std::fs;
use std::path::Path;

use toml_edit::{
    ArrayOfTables,
    DocumentMut, 
    Item, 
    Table, 
    Value,
};

use crate::resolver::TorchSource;
use crate::errors::BootstrapError;

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

fn insert_source_array(
    doc: &mut DocumentMut,
    source: &TorchSource,
) -> Result<(), BootstrapError> {

    let tool = doc
        .entry("tool")
        .or_insert(Item::Table(Table::new()))
        .as_table_mut()
        .ok_or_else(|| BootstrapError::Other("Failed to get mutable tool table".into()))?;

    let poetry = tool
        .entry("poetry")
        .or_insert(Item::Table(Table::new()))
        .as_table_mut()
        .ok_or_else(|| BootstrapError::Other("Failed to get mutable poetry table".into()))?;

    let entry = poetry
        .entry("source")
        .or_insert(Item::ArrayOfTables(ArrayOfTables::new()));

    let arr = entry
        .as_array_of_tables_mut()
        .ok_or_else(|| BootstrapError::Other("Failed to get mutable ArrayOfTables".into()))?;

    let already_exists = arr.iter().any(|tbl| {
        tbl.get("name").and_then(Item::as_str) == Some(&source.source) &&
        tbl.get("url").and_then(Item::as_str) == Some(&source.url)
    });

    if already_exists {
        println!(
            "Source '{}' with URL '{}' already exists in the pyproject.toml.",
            source.source, source.url
        );
        return Ok(());
    }

    let mut tbl = Table::new();
    tbl["name"] = Value::from(source.source.clone()).into();
    tbl["url"] = Value::from(source.url.clone()).into();
    tbl["priority"] = Value::from("explicit").into();

    arr.push(tbl);
    Ok(())
}

pub fn patch_pyproject(
    pyproject_path: &Path,
    source: &TorchSource,
) -> Result<(), BootstrapError> {
    let content = fs::read_to_string(pyproject_path)?;
    let mut doc: DocumentMut = content.parse()?;

    insert_source_array(&mut doc, source)?;
    fs::write(pyproject_path, doc.to_string())?;
    Ok(())
}

pub fn patch_pyproject_to_output(
    input: &Path,
    output: &Path,
    source: &TorchSource,
) -> Result<(), BootstrapError> {
    let content = fs::read_to_string(input)?;
    let mut doc: DocumentMut = content.parse()?;

    insert_source_array(&mut doc, source)?;
    fs::write(output, doc.to_string())?;
    Ok(())
}
