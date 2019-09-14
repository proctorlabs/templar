mod common;

use crate::*;
use std::collections::HashMap;

macro_rules! builtin_functions {
    ($( $( #[ $attr:meta ] )* $name:literal : $method:ident),*) => {
        pub fn default_functions() -> HashMap<String, Arc<Function>> {
            let mut res = HashMap::new();
            $(
                $( #[ $attr ] )*
                res.insert($name.into(), Arc::new(common::$method) as Arc<Function>);
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
