mod common;

use crate::*;
use std::collections::HashMap;

macro_rules! builtin_filters {
    ($( $( #[ $attr:meta ] )* $name:literal : $method:ident),*) => {
        pub fn default_filters() -> HashMap<String, Arc<Filter>> {
            let mut res = HashMap::new();
            $(
                $( #[ $attr ] )*
                res.insert($name.into(), Arc::new(common::$method) as Arc<Filter>);
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
    "key":key,
    "escape_html":escape_html,
    "e":escape_html
}
