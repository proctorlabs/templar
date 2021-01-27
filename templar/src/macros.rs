/*!
Various useful utility macros for Templar
*/

/// data_unwrap!() safely unwraps a `Data` struct into a Document value.
///
/// If the Data is failed or empty, the data is immediately returned. Because of this,
/// this macro can only be used in functions that return `Data`.
#[macro_export]
macro_rules! data_unwrap {
    ($data:expr) => {{
        let d = $data;
        if d.is_empty() || d.is_failed() {
            return d;
        }
        d
    }};
}

/// render_unwrap!() safely renders a Data struct into a string.
///
/// If the Data is failed or empty, the data is immediately returned. Because of this,
/// this macro can only be used in functions that return `Data`.
#[macro_export]
macro_rules! render_unwrap {
    ($data:expr) => {{
        match $data.render() {
            Ok(s) => s,
            Err(e) => return e.into(),
        }
    }};
}
