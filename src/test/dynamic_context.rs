use super::*;

const DYN_CONTEXT: &str = r#"---
somedict:
  static: value
  dynamic: "{{ somedict.static | upper }}"
"#;

#[test]
fn run_dynamic_context() -> Result<()> {
    let template = Templar::global().parse_yaml(DYN_CONTEXT)?;
    let context = StandardContext::new(template);
    let tmpl: Template = Templar::global().parse_template("{{ somedict.dynamic }}")?;
    let result = tmpl.render(&context)?;
    assert_eq!(result, "VALUE".to_string());
    Ok(())
}
