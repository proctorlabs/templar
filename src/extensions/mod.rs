mod builtin_filters;
mod builtin_functions;
mod default_ops;

use crate::*;
use std::collections::HashMap;
use unstructured::Document;

pub type TemplarResult = Result<Document>;

pub type GenericFilter<'de, T, U, V> = fn(T, U) -> Result<V>;
pub type Filter = dyn Fn(TemplarResult, TemplarResult) -> TemplarResult + Send + Sync;

pub type GenericFunction<'de, T, U> = fn(T) -> Result<U>;
pub type Function = dyn Fn(TemplarResult) -> TemplarResult + Send + Sync;

macro_rules! builtin_filters {
    ($( $( #[ $attr:meta ] )* $name:literal : $method:ident),*) => {
        pub fn default_filters() -> HashMap<String, Arc<Filter>> {
            let mut res = HashMap::new();
            $(
                $( #[ $attr ] )*
                res.insert($name.into(), Arc::new(builtin_filters::$method) as Arc<Filter>);
            )*
            res
        }
    };
}

macro_rules! builtin_functions {
    ($( $( #[ $attr:meta ] )* $name:literal : $method:ident),*) => {
        pub fn default_functions() -> HashMap<String, Arc<Function>> {
            let mut res = HashMap::new();
            $(
                $( #[ $attr ] )*
                res.insert($name.into(), Arc::new(builtin_functions::$method) as Arc<Function>);
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

builtin_filters! {
    //operations, these ops can also be used like filters
    "add":add,
    "subtract":subtract,
    "divide":divide,
    "multiply":multiply,
    "mod":modulus,
    "and":and,
    "or":or,
    "equals":equals,
    "not_equals":not_equals,
    "greater_than":greater_than,
    "greater_than_equals":greater_than_equals,
    "less_than":less_than,
    "less_than_equals":less_than_equals,
    "concat":concat,

    //common
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
