/*!
# Filters
 */

mod common;

use crate::*;
use std::collections::HashMap;

/// This is the definition used when adding filters to Templar
pub type Filter = dyn Fn(Data, Data) -> Data + Send + Sync;

macro_rules! builtin_filters {
    ($( $( #[ $attr:meta ] )* $name:literal : $method:path ; )*) => {
        pub(crate) fn default_filters() -> HashMap<String, Arc<Filter>> {
            let mut res = HashMap::new();
            $(
                $( #[ $attr ] )*
                res.insert($name.into(), Arc::new($method) as Arc<Filter>);
            )*
            res
        }
    };
}

builtin_filters! {
    "require": common::require;
    "default": common::default;
    "length": common::length;
    "lower": common::lower;
    "upper": common::upper;
    "trim": common::trim;
    "split": common::split;
    "index": common::index;
    "join": common::join;
    "string": common::string;
    "key": common::key;
    "escape_html": common::escape_html;
    "e": common::escape_html;

    #[cfg(feature = "yaml-extension")]
    "yaml": common::yaml;
    #[cfg(feature = "yaml-extension")]
    "yml": common::yaml;
    #[cfg(feature = "json-extension")]
    "json": common::json;
    #[cfg(feature = "base64-extension")]
    "base64": common::base64;
}
