mod builtin_filters;
mod builtin_functions;

use crate::*;
use std::collections::HashMap;
use std::sync::Arc;
use unstructured::Document;

pub type TemplarResult = Result<Document>;
pub type Filter = fn(TemplarResult, TemplarResult) -> TemplarResult;
pub type Function = fn(TemplarResult) -> TemplarResult;

pub fn default_filters() -> HashMap<String, Arc<Filter>> {
    let mut res = HashMap::new();
    res.insert("length".into(), Arc::new(builtin_filters::length as Filter));
    res.insert("lower".into(), Arc::new(builtin_filters::lower as Filter));
    res.insert("upper".into(), Arc::new(builtin_filters::upper as Filter));
    res.insert("trim".into(), Arc::new(builtin_filters::trim as Filter));
    res.insert("yaml".into(), Arc::new(builtin_filters::yaml as Filter));
    res.insert("yml".into(), Arc::new(builtin_filters::yaml as Filter));
    res.insert("json".into(), Arc::new(builtin_filters::json as Filter));
    res
}

pub fn default_functions() -> HashMap<String, Arc<Function>> {
    let mut res = HashMap::new();
    res.insert("nop".into(), Arc::new(builtin_functions::nop as Function));
    res
}
