use super::*;

use std::convert::TryInto;

const BASIC: &str = r#"---
somedict:
  val1: value
  val2: "{{ file('README.md') | split | index(0) }}"
"#;

#[test]
fn parse_yml_template() -> Result<()> {
    let template = Templar::global().parse_yaml(BASIC)?;
    let context = Context::new_standard({
        let mut doc = Document::default();
        doc["one"]["two"]["three"] = "val".into();
        doc
    });
    let tmpl: Template = template.get_path(&["somedict", "val2"]).try_into()?;
    let result = tmpl.render(&context)?;
    println!("Result: {}", result);
    Ok(())
}
