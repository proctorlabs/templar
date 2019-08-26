use crate::*;

const BASIC: &str = include_str!("basic.yml");

#[test]
fn parse_yml_template() -> Result<()> {
    let template = Templar::global().parse_yaml(BASIC)?;
    let context = StandardContext::new({
        let mut doc = Document::default();
        doc["one"]["two"]["three"] = "val".into();
        doc
    });
    let result = template
        .get_path(vec!["somedict", "val2"])?
        .render(&context)?;
    println!("Result: {}", result);
    Ok(())
}
