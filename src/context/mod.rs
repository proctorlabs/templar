use crate::*;
use std::fmt::Debug;

mod dynamic;
pub use dynamic::*;

mod scoped;
mod standard;

pub(crate) use scoped::ScopedContext;
pub use standard::StandardContext;

#[cfg(feature = "shared-context")]
mod shared;
#[cfg(feature = "shared-context")]
pub use shared::SharedContext;

/// The primary context trait
pub trait Context: Sized {
    #[doc(hidden)]
    fn set_path_inner(&self, path: &[&Document], doc: ContextMapValue) -> Result<()>;
    #[doc(hidden)]
    fn get_path_inner(&self, path: &[&Document], ctx: &impl Context) -> Data;
    #[doc(hidden)]
    fn wrap(&self) -> ContextWrapper;

    /// Merge the data into the root context
    #[inline]
    fn merge<T: Into<Document>>(&self, doc: T) -> Result<()> {
        match doc.into() {
            Document::Map(m) => {
                for (k, v) in m.into_iter() {
                    let orig = self.get_path(&[&k]).into_result().unwrap_or_default();
                    self.set_path(&[&k], orig + v)?;
                }
                Ok(())
            }
            _ => Err(TemplarError::ParseFailure(
                "Cannot merge a non-mapping into the root context".into(),
            )),
        }
    }

    /// Merge the data into the context at the specified path
    #[inline]
    fn merge_path<T>(&self, path: &[&Document], doc: T) -> Result<()>
    where
        Document: From<T>,
    {
        let orig = self.get_path(path).into_result()?;
        self.set_path::<Document>(path, orig + Document::from(doc))?;
        Ok(())
    }

    /// Set the root context value
    #[inline]
    fn set<T: Into<ContextMapValue>>(&self, doc: T) -> Result<()> {
        self.set_path_inner(&[], doc.into())
    }

    /// Enter a new scope
    #[inline]
    fn create_scope(&self) -> ScopedContext {
        ScopedContext::new(self.wrap())
    }

    /// Set the value at the specified path
    #[inline]
    fn set_path<T: Into<ContextMapValue>>(&self, path: &[&Document], doc: T) -> Result<()> {
        self.set_path_inner(path, doc.into())
    }

    /// Get the root context value
    #[inline]
    fn get(&self) -> Data {
        self.get_path_inner(&[], self)
    }

    /// Get the value at the path
    #[inline]
    fn get_path(&self, path: &[&Document]) -> Data {
        self.get_path_inner(path, self)
    }
}

#[derive(Debug)]
pub enum ContextWrapper<'a> {
    Standard(&'a StandardContext),
    Shared(&'a SharedContext),
    Scope(&'a ScopedContext<'a>),
}

impl<'a> Context for ContextWrapper<'a> {
    fn set_path_inner(&self, path: &[&Document], doc: ContextMapValue) -> Result<()> {
        match self {
            Self::Standard(c) => c.set_path_inner(path, doc),
            Self::Shared(c) => c.set_path_inner(path, doc),
            Self::Scope(c) => c.set_path_inner(path, doc),
        }
    }

    fn get_path_inner(&self, path: &[&Document], ctx: &impl Context) -> Data {
        match self {
            Self::Standard(c) => c.get_path_inner(path, ctx),
            Self::Shared(c) => c.get_path_inner(path, ctx),
            Self::Scope(c) => c.get_path_inner(path, ctx),
        }
    }

    fn wrap(&self) -> ContextWrapper {
        match self {
            Self::Standard(c) => c.wrap(),
            Self::Shared(c) => c.wrap(),
            Self::Scope(c) => c.wrap(),
        }
    }
}
