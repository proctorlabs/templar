#[macro_use]
extern crate lazy_static;

pub(crate) use error::*;
use std::{collections::HashMap, sync::Arc};
pub use {
    self::{
        context::{Context, SharedContext, StandardContext},
        extensions::{Filter, Function, GenericFilter, GenericFunction, TemplarResult},
        nodes::Node,
        templar::*,
        template::{Template, TemplateTree},
    },
    unstructured::Document,
};

pub mod error;

#[cfg(test)]
mod test;

mod context;
mod extensions;
mod nodes;
mod parser;
mod templar;
mod template;
