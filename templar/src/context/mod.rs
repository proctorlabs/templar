use crate::*;
use std::fmt::Debug;

mod dynamic;
pub use dynamic::*;

mod scoped;
mod standard;

pub(crate) use scoped::ScopedContext;
pub use standard::StandardContext;

// #[cfg(feature = "shared-context")]
// mod shared;
// #[cfg(feature = "shared-context")]
// pub use shared::SharedContext;

/// The primary context trait
pub trait Context: Sized {
    #[doc(hidden)]
    fn set_path_inner(&self, path: &[&InnerData], doc: ContextMapValue) -> Result<()>;
    #[doc(hidden)]
    fn get_path_inner(&self, path: &[&InnerData], ctx: &impl Context) -> Data;
    #[doc(hidden)]
    fn wrap(&self) -> ContextWrapper;

    /// Merge the data into the root context
    #[inline]
    fn merge<T: Into<InnerData>>(&self, doc: T) -> Result<()> {
        match doc.into() {
            InnerData::Map(m) => {
                for (k, v) in m.into_iter() {
                    let orig = self.get_path(&[&k]).into_result()?;
                    self.set_path(&[&k], orig.into_inner() + v)?;
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
    fn merge_path<T>(&self, path: &[&InnerData], doc: T) -> Result<()>
    where
        InnerData: From<T>,
    {
        let orig = self.get_path(path).into_result()?;
        self.set_path::<InnerData>(path, orig.into_inner() + InnerData::from(doc))?;
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
    fn set_path<T: Into<ContextMapValue>>(&self, path: &[&InnerData], doc: T) -> Result<()> {
        self.set_path_inner(path, doc.into())
    }

    /// Get the root context value
    #[inline]
    fn get(&self) -> Data {
        self.get_path_inner(&[], self)
    }

    /// Get the value at the path
    #[inline]
    fn get_path(&self, path: &[&InnerData]) -> Data {
        self.get_path_inner(path, self)
    }
}

#[derive(Debug)]
pub enum ContextWrapper<'a> {
    Standard(&'a StandardContext),
    #[cfg(feature = "shared-context")]
    // Shared(&'a SharedContext),
    Scope(&'a ScopedContext<'a>),
}

impl<'a> Context for ContextWrapper<'a> {
    fn set_path_inner(&self, path: &[&InnerData], doc: ContextMapValue) -> Result<()> {
        match self {
            Self::Standard(c) => c.set_path_inner(path, doc),
            // #[cfg(feature = "shared-context")]
            // Self::Shared(c) => c.set_path_inner(path, doc),
            Self::Scope(c) => c.set_path_inner(path, doc),
        }
    }

    fn get_path_inner(&self, path: &[&InnerData], ctx: &impl Context) -> Data {
        let res = match self {
            Self::Standard(c) => c.get_path_inner(path, ctx),
            #[cfg(feature = "shared-context")]
            // Self::Shared(c) => c.get_path_inner(path, ctx),
            Self::Scope(c) => c.get_path_inner(path, ctx),
        };
        println!("---> {:?}", res);
        res
    }

    fn wrap(&self) -> ContextWrapper {
        match self {
            Self::Standard(c) => c.wrap(),
            #[cfg(feature = "shared-context")]
            // Self::Shared(c) => c.wrap(),
            Self::Scope(c) => c.wrap(),
        }
    }
}
