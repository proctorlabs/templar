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
    fn set_path(&self, path: &[Document], doc: ContextMapValue) -> Result<()>;
    fn get_path(&self, path: &[Document], ctx: &Context) -> Data;
}

#[derive(Debug)]
pub struct Context<'a> {
    dispatcher: Box<dyn ContextDispatcher + 'a>,
}

impl<'a> Context<'a> {
    pub(crate) fn create_scope(&'a self) -> Context<'a> {
        Context {
            dispatcher: Box::new(ScopedContext::new(&self.dispatcher)),
        }
    }
}

impl Context<'_> {
    pub fn new_standard<T: Into<ContextMapValue>>(initial_value: T) -> Self {
        Context {
            dispatcher: Box::new(StandardContext::new(initial_value)),
        }
    }

    #[cfg(feature = "shared-context")]
    pub fn new_shared<T: Into<ContextMapValue>>(initial_value: T) -> Self {
        Context {
            dispatcher: Box::new(SharedContext::new(initial_value)),
        }
    }

    #[inline]
    pub fn merge(&self, doc: Document) -> Result<()> {
        let orig = self.get().result()?;
        self.set(orig + doc)?;
        Ok(())
    }

    #[inline]
    pub fn merge_path(&self, path: &[Document], doc: Document) -> Result<()> {
        let orig = self.get_path(path).result()?;
        self.set_path(path, (orig + doc).into())?;
        Ok(())
    }

    #[inline]
    pub fn set(&self, doc: Document) -> Result<()> {
        self.dispatcher.set_path(&[], doc.into())
    }

    #[inline]
    pub fn set_path(&self, path: &[Document], doc: ContextMapValue) -> Result<()> {
        self.dispatcher.set_path(path, doc)
    }

    #[inline]
    pub fn get(&self) -> Data {
        self.dispatcher.get_path(&[], &self)
    }

    #[inline]
    pub fn get_path(&self, path: &[Document]) -> Data {
        self.dispatcher.get_path(path, &self)
    }
}
