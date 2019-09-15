use templar::*;

const TEMPLATE: &str = r#"
{#- This is a comment, the '-' on either side will eat whitespace too -#}
The value of some.string is: {{ some.string }}
{#- using '-' on only the left side will eat the newline prior to this so that there isn't a blank line in between #}
The value of some.number is: {{ some.number -}}
"#;

fn main() -> Result<(), TemplarError> {
    // Parse the template using the global instance, this is suitable for most purposes
    let template = Templar::global().parse(TEMPLATE)?;

    // Create a context and set some data
    let mut data: Document = Document::default();
    data["some"]["string"] = "Hello World!".into();
    data["some"]["number"] = 42i64.into();
    let context = Context::new_standard(data);

    // Render a template using the context creates
    println!("{}", template.render(&context)?);

    // -- The output of this looks like below
    // The value of some.string is: Hello World!
    // The value of some.number is: 42

    Ok(())
}
