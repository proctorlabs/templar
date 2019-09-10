use super::*;

#[allow(clippy::unnecessary_fold)]
#[test]
fn call_generic_function() -> Result<()> {
    let mut builder = TemplarBuilder::default();
    builder
        .add_generic_function("sum", |args: Vec<i64>| {
            Ok(args.iter().fold(0, |acc, i| acc + i))
        })
        .add_generic_function("add_two_args", |args: (i64, i64)| Ok(args.0 + args.1));
    let templar = builder.build();
    let template = templar.parse_expression("sum([5,add_two_args(5,3),3])")?;
    let context = Context::new_standard(Document::Unit);
    let res: i64 = template.exec(&context)?.try_into().unwrap();
    assert_eq!(res, 16 as i64);
    Ok(())
}

#[test]
fn call_generic_filter() -> Result<()> {
    let mut builder = TemplarBuilder::default();
    builder.add_generic_filter("add_single", |inc: i64, arg: i64| Ok(inc + arg));
    let templar = builder.build();
    let template = templar.parse_expression("5 + 3 | add_single(2)")?;
    let context = Context::new_standard(Document::Unit);
    let res: i64 = template.exec(&context)?.try_into().unwrap();
    assert_eq!(res, 10 as i64);
    Ok(())
}
