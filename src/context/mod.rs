use std::fmt::Debug;

mod dynamic;
pub use dynamic::*;

mod scoped;
pub(crate) use scoped::ScopedContext;

mod standard;
pub use standard::StandardContext;

#[cfg(feature = "shared-context")]
mod shared;
#[cfg(feature = "shared-context")]
pub use shared::SharedContext;

pub trait Context: Debug {
    fn merge(&self, doc: Document) {
        let orig = self.get();
        self.set(orig + doc);
    }

    #[inline]
    fn set(&self, doc: Document) {
        self.set_path(&[], doc);
    }

    fn set_path(&self, path: &[Document], doc: Document);

    #[inline]
    fn get(&self) -> Document {
        self.get_path(&[])
    }

    fn get_path(&self, path: &[Document]) -> Document;
}
