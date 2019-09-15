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

/// Easily define simple filters that expect certain types
///
/// ```t
/// fn hello_world(inc: i64 | I64, args: i64 | I64) -> i64 {
///     inc + args
/// }
/// ```
#[macro_export]
macro_rules! templar_filter {
    ( $( fn $name:ident ( $left:ident : $left_ty:ty | $left_var:ident, $right:ident : $right_ty:ty | $right_var:ident) -> $out:ty { $( $tail:tt )* } )* ) => {
        $(
            pub fn $name(left: Data, right: Data) -> Data {
                match (left.result(), right.result()) {
                    (Err(e), _) | (_, Err(e)) => return e.into(),
                    (Ok(l), Ok(r)) => {
                        let $left: $left_ty = match l {
                            Document::$left_var(i) => i,
                            _ => return TemplarError::RenderFailure(format!("Expected {} for incoming filter data", stringify!($left_ty))).into(),
                        }.into();
                        let $right: $right_ty = match r {
                            Document::$right_var(i) => i,
                            _ => return TemplarError::RenderFailure(format!("Expected {} for filter args", stringify!($right_ty))).into(),
                        }.into();
                        <$out>::from({
                            $( $tail )*
                        }).into()
                    }
                }
            }
        )*
    };
}

/// Easily define simple funtions that expect certain types
///
/// ```t
/// fn hello_world(args: i64 | I64) -> i64 {
///     args + 5
/// }
/// ```
#[macro_export]
macro_rules! templar_function {
    ( $( fn $name:ident ( $left:ident : $left_ty:ty | $left_var:ident) -> $out:ty { $( $tail:tt )* } )* ) => {
        $(
            pub fn $name(left: Data) -> Data {
                left.result() {
                    Err(e) => return e.into(),
                    Ok(l) => {
                        let $left: $left_ty = match l {
                            Document::$left_var(i) => i,
                            _ => return TemplarError::RenderFailure(format!("Expected {} for function args", stringify!($left_ty))).into(),
                        }.into();
                        <$out>::from({
                            $( $tail )*
                        }).into()
                    }
                }
            }
        )*
    };
}
