use templar::{Data, TemplarError};
use unstructured::Document;

#[macro_use]
extern crate templar_macros;

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
    filter_in.replace(&old, &new).into()
}
