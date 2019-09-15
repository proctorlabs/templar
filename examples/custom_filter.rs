use templar::*;

const TEMPLATE: &str = r#"
{{- `I'm CRAZY!!!` | repeater(5) -}}
"#;

// This macro isn't necessary, but it makes it easy if your method is expecting specific data types.
// The extra identifier after the '|' is the unstructured::Document variant we are expecting.
// This macro will also throw sane errors if incoming data types are not what is expected.
// If you need to throw an error inside your macro definition, you can return TemplarError::RenderFailure(format!("Message")).into()
// and these errors will propagate up to the render() or exec() calls to the template.
// See the `Filter` type in lib.rs if you can't or don't want to use the macro.
templar_filter! {
    fn repeater(inc: String | String, repeat: i64 | I64) -> String {
        let mut to_repeat = String::new();
        for _ in 0..repeat {
            to_repeat.push_str(&format!("{}\n", inc));
        }
        to_repeat
    }
}

fn main() -> Result<(), TemplarError> {
    // Since we need to customize our instance, we can't use the global instance. We'll use a builder.
    let mut builder = TemplarBuilder::default();
    builder.add_filter("repeater", repeater);
    let templar = builder.build();

    // Parse the template using our customized instance
    let template = templar.parse(TEMPLATE)?;

    // Create an empty context
    let context = Context::new_standard(Document::Unit);

    // Render a template using the context creates
    println!("{}", template.render(&context)?);

    // -- The output of this looks like below
    // I'm CRAZY!!!
    // I'm CRAZY!!!
    // I'm CRAZY!!!
    // I'm CRAZY!!!
    // I'm CRAZY!!!

    Ok(())
}
