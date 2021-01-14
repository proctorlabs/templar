use templar::{Data, StandardContext, TemplarBuilder, TemplarError, InnerData};

#[macro_use]
extern crate templar_macros;

const TEMPLATE: &str = r#"
{{- `I'm CRAZY!!!` | repeater(5) -}}
"#;

#[templar_filter]
fn repeater(inc: String, count: i64) -> String {
    let mut inc = inc;
    inc.push('\n');
    inc.repeat(count as usize).trim().into()
}

#[test]
fn custom_filter_repeater() -> Result<(), TemplarError> {
    let mut builder = TemplarBuilder::default();
    builder.add_filter("repeater", repeater);
    let templar = builder.build();
    let template = templar.parse(TEMPLATE)?;
    let context = StandardContext::new();
    let result = template.render(&context)?;
    assert_eq!(
        result,
        r#"I'm CRAZY!!!
I'm CRAZY!!!
I'm CRAZY!!!
I'm CRAZY!!!
I'm CRAZY!!!"#
    );
    Ok(())
}

#[templar_filter]
fn function_attr(test: i32, _test2: i32) -> Result<String, Box<dyn std::error::Error>> {
    assert_eq!(10, test);
    Ok("Test".into())
}

#[test]
fn basic() -> Result<(), Box<dyn std::error::Error>> {
    function_attr(10.into(), 10.into()).into_result()?;
    __function_attr_inner__(10, 10)?;
    Ok(())
}

#[templar_function]
pub fn replace(filter_in: String, old: String, new: String) -> String {
    filter_in.replace(&old, &new)
}
