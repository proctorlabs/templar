/*!
# Functions
 */

mod common;

use crate::*;
use std::collections::HashMap;

/// This is the definition used when adding functions to Templar
pub type Function = dyn Fn(Data) -> Data + Send + Sync;

macro_rules! builtin_functions {
    ($( $( #[ $attr:meta ] )* $name:literal : $method:path ; )*) => {
        pub(crate) fn default_functions() -> HashMap<String, Arc<Function>> {
            let mut res = HashMap::new();
            $(
                $( #[ $attr ] )*
                res.insert($name.into(), Arc::new($method) as Arc<Function>);
            )*
            res
        }
    };
}

builtin_functions! {
    "file": common::file;
    "env": common::env;
    "script": common::script;
    "command": common::command;

    #[cfg(feature = "json-extension")]
    "json": common::json;
    #[cfg(feature = "yaml-extension")]
    "yaml": common::yaml;
    #[cfg(feature = "yaml-extension")]
    "yml": common::yaml;
}
