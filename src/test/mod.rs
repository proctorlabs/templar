use crate::*;
use std::convert::TryInto;

const BASIC: &str = include_str!("basic.yml");

#[test]
fn parse_yml_template() -> Result<()> {
    let template = Templar::global().parse_yaml(BASIC)?;
    let context = StandardContext::new({
        let mut doc = Document::default();
        doc["one"]["two"]["three"] = "val".into();
        doc
    });
    let tmpl: Template = template.get_path(&["somedict", "val2"]).try_into()?;
    let result = tmpl.render(&context)?;
    println!("Result: {}", result);
    Ok(())
}

#[test]
fn parse_expression_direct() -> Result<()> {
    let tmpl = Templar::global().parse_expression(" 1 + 2 ")?;
    let context = StandardContext::new({
        let mut doc = Document::default();
        doc["one"]["two"]["three"] = "val".into();
        doc
    });
    let result = tmpl.render(&context)?;
    println!("Result: {}", result);
    Ok(())
}
