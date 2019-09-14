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
        d.unwrap()
    }};
}

/// data_unwrap_into!() safely unwraps a `Data` struct into a Document value of the specified type.
///
/// If the Data is failed or empty, the data is immediately returned. Because of this,
/// this macro can only be used in functions that return `Data`.
#[macro_export]
macro_rules! data_unwrap_into {
    ($type:ident : $data:expr) => {{
        match $data.result() {
            Ok(Document::$type(t)) => t,
            Ok(o) => return o.into(),
            Err(e) => return e.into(),
        }
    }};
}
