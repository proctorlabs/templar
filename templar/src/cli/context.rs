use super::*;
use std::path::PathBuf;
use templar::Templar;
use unstructured::Document;

pub fn build_context(options: &Command) -> Result<StandardContext> {
    let ctx = StandardContext::new();
    for file in options.dynamic_input.iter() {
        let doc = parse_data(file)?;
        let tree: TemplateTree = Templar::global().parse(&doc)?;
        ctx.set(tree)?;
    }
    for file in options.input.iter() {
        ctx.merge(parse_data(file)?)?;
    }
    for setter in options.set.iter() {
        ctx.set_path(&[&setter.0.to_string().into()], setter.1.to_string())?;
    }
    Ok(ctx)
}

fn parse_data(path: &PathBuf) -> Result<Document> {
    let contents = read_file(path)?;
    let ext: String = path
        .extension()
        .map(|ext| ext.to_string_lossy().to_lowercase())
        .unwrap_or_default();
    Ok(match &ext as &str {
        "js" | "json" => serde_json::from_str(&contents).wrap()?,
        "yml" | "yaml" => serde_yaml::from_str(&contents).wrap()?,
        "xml" => serde_xml_rs::from_str(&contents)
            .map_err(|e| TemplarError::RenderFailure(format!("{:?}", e)))?,
        "toml" => toml::from_str(&contents).wrap()?,
        _ => serde_json::from_str(&contents).wrap()?,
    })
}
