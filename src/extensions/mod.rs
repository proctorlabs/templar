mod filters;
mod functions;

use crate::*;
use std::collections::HashMap;
use unstructured::Document;

/// This is an execution result
pub type TemplarResult = Result<Document>;

/// Type used for generic filters when generics are enabled
pub type GenericFilter<'de, T, U, V> = fn(T, U) -> Result<V>;
/// Filter definition
pub type Filter = dyn Fn(Data, Data) -> Data + Send + Sync;

/// Type used for generic functions when generics are enabled
pub type GenericFunction<'de, T, U> = fn(T) -> Result<U>;
/// Function definition
pub type Function = dyn Fn(TemplarResult) -> TemplarResult + Send + Sync;

macro_rules! builtin_filters {
    ($( $( #[ $attr:meta ] )* $name:literal : $method:ident),*) => {
        pub fn default_filters() -> HashMap<String, Arc<Filter>> {
            let mut res = HashMap::new();
            $(
                $( #[ $attr ] )*
                res.insert($name.into(), Arc::new(filters::$method) as Arc<Filter>);
            )*
            res
        }
    };
}

builtin_filters! {
    "require":require,
    "default":default,
    "length":length,
    "lower":lower,
    "upper":upper,
    "trim":trim,
    #[cfg(feature = "yaml-extension")]
    "yaml":yaml,
    #[cfg(feature = "yaml-extension")]
    "yml":yaml,
    #[cfg(feature = "json-extension")]
    "json":json,
    "split":split,
    "index":index,
    #[cfg(feature = "base64-extension")]
    "base64":base64,
    "join":join,
    "string":string,
    "key":key
}

macro_rules! builtin_functions {
    ($( $( #[ $attr:meta ] )* $name:literal : $method:ident),*) => {
        pub fn default_functions() -> HashMap<String, Arc<Function>> {
            let mut res = HashMap::new();
            $(
                $( #[ $attr ] )*
                res.insert($name.into(), Arc::new(functions::$method) as Arc<Function>);
            )*
            res
        }
    };
}

builtin_functions! {
    #[cfg(feature = "json-extension")]
    "json":json,
    #[cfg(feature = "yaml-extension")]
    "yaml":yaml,
    #[cfg(feature = "yaml-extension")]
    "yml":yaml,
    "file":file,
    "env":env,
    "script":script,
    "command":command
}
