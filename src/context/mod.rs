use crate::*;
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
    fn merge(&self, doc: Document) -> Result<()> {
        let orig = self.get().result()?;
        self.set(orig + doc)?;
        Ok(())
    }

    #[inline]
    fn set(&self, doc: Document) -> Result<()> {
        self.set_path(&[], doc.into())
    }

    fn set_path(&self, path: &[Document], doc: ContextMapValue) -> Result<()>;

    #[inline]
    fn get(&self) -> Data {
        self.get_path(&[])
    }

    fn get_path(&self, path: &[Document]) -> Data;
}
