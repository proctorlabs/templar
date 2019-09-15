#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;
use templar::error::*;
use templar::{Context, Templar, Template};

static EXPR: &str = r#"
This is a template.

{# It includes comments #}
{{ `and expressions` }}
{{ "THAT CAN CALL FILTERS" | lower }}
"#;

fn exec_expression(template: &Template, context: &Context) -> Result<()> {
    template.exec(context)?;
    Ok(())
}

fn criterion_benchmark(c: &mut Criterion) {
    let template = Templar::global().parse_template(EXPR).unwrap();
    let context = Context::new_standard(unstructured::Document::Unit);
    c.bench_function("Execute a simple expression", |b| {
        b.iter(|| exec_expression(black_box(&template), black_box(&context)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
