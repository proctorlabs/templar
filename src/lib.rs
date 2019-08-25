#[macro_use]
extern crate lazy_static;

pub(crate) use error::*;
use std::collections::HashMap;
use std::sync::Arc;

pub use context::{SharedContext, StandardContext};
pub use extensions::{Filter, Function, TemplarResult};
pub use nodes::Node;
pub use template::Template;
pub use unstructured::Document;

#[cfg(test)]
mod test;

mod context;
pub mod error;
mod extensions;
mod nodes;
mod parser;
mod template;

lazy_static! {
    static ref GLOBAL: Templar = { Templar::default() };
}

pub struct Templar {
    functions: HashMap<String, Arc<Function>>,
    filters: HashMap<String, Arc<Filter>>,
}

impl Default for Templar {
    fn default() -> Templar {
        Templar {
            functions: extensions::default_functions(),
            filters: extensions::default_filters(),
        }
    }
}

impl Templar {
    #[inline]
    pub fn global() -> &'static Templar {
        &GLOBAL
    }

    #[inline]
    pub fn parse(&self, _: &Document) {}

    #[inline]
    pub fn parse_str(&self, val: &str) -> Result<Template> {
        self.parse_template(val)
    }

    #[inline]
    pub fn parse_json(&self) {}

    #[inline]
    pub fn parse_yaml(&self) {}
}
