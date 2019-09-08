use std::fmt::Debug;
use unstructured::Document;

mod scoped;
pub(crate) use scoped::ScopedContext;

mod standard;
pub use standard::StandardContext;

#[cfg(feature = "shared-context")]
mod shared;
#[cfg(feature = "shared-context")]
pub use shared::SharedContext;

pub trait Context: Debug {
    fn merge(&self, doc: Document);

    fn set(&self, doc: Document);

    fn set_path(&self, path: &[&String], doc: Document);

    fn get(&self) -> Document;

    fn get_path(&self, path: &[&String]) -> Document;
}
