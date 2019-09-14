use crate::*;
use std::fmt::Debug;

mod dynamic;
pub use dynamic::*;

mod scoped;
mod standard;

use {scoped::ScopedContext, standard::StandardContext};

#[cfg(feature = "shared-context")]
mod shared;
#[cfg(feature = "shared-context")]
use shared::SharedContext;

pub trait ContextDispatcher: Debug {
    fn set_path(&self, path: &[&Document], doc: ContextMapValue) -> Result<()>;
    fn get_path(&self, path: &[&Document], ctx: &Context) -> Data;
}

/// This is the context used for template rendering. It can hold either static or dynamic
/// data.
#[derive(Debug)]
pub struct Context<'a> {
    dispatcher: Box<dyn ContextDispatcher + 'a>,
}

impl<'a> Context<'a> {
    /// Create a new scope for this context. Any new data set here will not affect the parent
    /// context but will be available to all data rendered inside the scope.
    pub(crate) fn create_scope(&'a self) -> Self {
        Context {
            dispatcher: Box::new(ScopedContext::new(&self.dispatcher)),
        }
    }

    /// Create a new standard context. Use this when thread safety is not needed.
    pub fn new_standard<T: Into<ContextMapValue>>(initial_value: T) -> Context<'static> {
        Context {
            dispatcher: Box::new(StandardContext::new(initial_value)),
        }
    }

    /// Create a new shared context. This is a thread safe context.
    #[cfg(feature = "shared-context")]
    pub fn new_shared<T: Into<ContextMapValue>>(initial_value: T) -> Context<'static> {
        Context {
            dispatcher: Box::new(SharedContext::new(initial_value)),
        }
    }

    /// Merge the data into the root context
    #[inline]
    pub fn merge<T: Into<Document>>(&self, doc: T) -> Result<()> {
        let orig = self.get().result()?;
        self.set(orig + doc.into())?;
        Ok(())
    }

    /// Merge the data into the context at the specified path
    #[inline]
    pub fn merge_path<T>(&self, path: &[&Document], doc: T) -> Result<()>
    where
        Document: From<T>,
    {
        let orig = self.get_path(path).result()?;
        self.set_path::<Document>(path, orig + Document::from(doc))?;
        Ok(())
    }

    /// Set the root context value
    #[inline]
    pub fn set<T: Into<ContextMapValue>>(&self, doc: T) -> Result<()> {
        self.dispatcher.set_path(&[], doc.into())
    }

    /// Set the value at the specified path
    #[inline]
    pub fn set_path<T: Into<ContextMapValue>>(&self, path: &[&Document], doc: T) -> Result<()> {
        self.dispatcher.set_path(path, doc.into())
    }

    /// Get the root context value
    #[inline]
    pub fn get(&self) -> Data {
        self.dispatcher.get_path(&[], &self)
    }

    /// Get the value at the path
    #[inline]
    pub fn get_path(&self, path: &[&Document]) -> Data {
        self.dispatcher.get_path(path, &self)
    }
}
