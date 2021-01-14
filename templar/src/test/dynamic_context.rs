use super::*;

const DYN_CONTEXT: &str = r#"---
somedict:
  static: value
  dynamic: "{{ somedict.static | upper }}"
  unknown: "{{ one.two.three }}"
"#;

#[test]
fn run_dynamic_context() -> Result<()> {
    let template = Templar::global().parse_yaml(DYN_CONTEXT)?;
    let context = StandardContext::new();
    context.set(template)?;
    let tmpl: Template = Templar::global().parse_template("{{ somedict.dynamic }}")?;
    let result = tmpl.render(&context)?;
    assert_eq!(result, "VALUE".to_string());
    Ok(())
}

#[test]
fn run_dynamic_context_new_value() -> Result<()> {
    let template = Templar::global().parse_yaml(DYN_CONTEXT)?;
    let context = StandardContext::new();
    context.set(template)?;
    let tmpl: Template = Templar::global()
        .parse_template("{{ one.two.three = 'HELLO!!' }}{{ somedict.unknown }}")?;
    let result = tmpl.render(&context)?;
    assert_eq!(result, "HELLO!!".to_string());
    Ok(())
}
