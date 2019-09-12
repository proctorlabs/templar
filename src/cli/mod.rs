use super::*;
use options::{Command, Options};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use templar::Templar;
use unstructured::Document;

mod options;

pub fn run() -> Result<()> {
    let options = Options::parse()?;
    match &options.command {
        Command::Expression { ref text } => run_expression(&options, text),
        Command::Template { ref file } => run_template(&options, file),
    }
}

fn read_file(path: &PathBuf) -> Result<String> {
    let mut file = File::open(path)?;
    let mut result = String::new();
    file.read_to_string(&mut result)?;
    Ok(result)
}

fn parse_file(path: &PathBuf) -> Result<Document> {
    let contents = read_file(path)?;
    let ext: String = path
        .extension()
        .map(|ext| ext.to_string_lossy().to_lowercase())
        .unwrap_or_default()
        .to_string();
    Ok(match &ext as &str {
        "js" | "json" => serde_json::from_str(&contents).wrap()?,
        "yml" | "yaml" => serde_yaml::from_str(&contents).wrap()?,
        "xml" => serde_xml_rs::from_str(&contents).wrap()?,
        "toml" => toml::from_str(&contents).wrap()?,
        _ => serde_json::from_str(&contents).wrap()?,
    })
}

fn build_context(options: &Options) -> Result<Context> {
    let ctx = Context::new_standard(Document::Unit);
    for file in options.input.iter() {
        ctx.merge(parse_file(file)?)?;
    }
    for setter in options.set.iter() {
        ctx.set_path(&[&setter.1.to_string().into()], setter.1.to_string())?;
    }
    Ok(ctx)
}

fn run_expression(options: &Options, text: &str) -> Result<()> {
    let ctx = build_context(options)?;
    let expr = Templar::global().parse_expression(text)?;
    write_output(options, expr.render(&ctx)?)
}

fn run_template(options: &Options, file: &PathBuf) -> Result<()> {
    let ctx = build_context(options)?;
    let template_contents = read_file(file)?;
    let template = Templar::global().parse_template(&template_contents)?;
    write_output(options, template.render(&ctx)?)
}

fn write_output(options: &Options, output: String) -> Result<()> {
    match options.output {
        Some(ref file) => {
            let mut f = File::create(file)?;
            f.write_all(output.as_bytes())?;
        }
        None => print!("{}", output),
    };
    Ok(())
}
