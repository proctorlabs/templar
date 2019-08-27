mod builtin_filters;
mod builtin_functions;
mod default_ops;

use crate::*;
use std::collections::HashMap;
use unstructured::Document;

pub type TemplarResult = Result<Document>;
pub type Filter = fn(TemplarResult, TemplarResult) -> TemplarResult;
pub type Function = fn(TemplarResult) -> TemplarResult;

macro_rules! builtin_filters {
    ($( $name:literal : $method:ident),*) => {
        pub fn default_filters() -> HashMap<String, Filter> {
            let mut res = HashMap::new();
            $(
                res.insert($name.into(), builtin_filters::$method as Filter);
            )*
            res
        }
    };
}

macro_rules! builtin_functions {
    ($( $name:literal : $method:ident),*) => {
        pub fn default_functions() -> HashMap<String, Function> {
            let mut res = HashMap::new();
            $(
                res.insert($name.into(), builtin_functions::$method as Function);
            )*
            res
        }
    };
}

builtin_functions! {
    "json":json,
    "yaml":yaml,
    "yml":yaml,
    "file":file,
    "env":env,
    "shell":shell
}

builtin_filters! {
    //ops
    "add":add,
    "subtract":subtract,
    "divide":divide,
    "multiply":multiply,
    "mod":modulus,

    //common
    "length":length,
    "lower":lower,
    "upper":upper,
    "trim":trim,
    "yaml":yaml,
    "yml":yaml,
    "json":json,
    "split":split,
    "index":index,
    "base64":base64,
    "join":join,
    "string":string,
    "key":key
}
