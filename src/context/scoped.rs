pub use super::*;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Debug, Clone)]
pub(crate) struct ScopedContext<'c>(&'c dyn Context, Rc<RefCell<Document>>);

impl<'c> ScopedContext<'c> {
    pub fn new(ctx: &'c dyn Context) -> Self {
        ScopedContext(ctx, Default::default())
    }
}

impl<'c> Context for ScopedContext<'c> {
    fn merge(&self, doc: Document) {
        self.1.borrow_mut().merge(doc);
    }

    fn set(&self, doc: Document) {
        self.1.borrow_mut().set(doc);
    }

    fn set_path(&self, path: &[&String], doc: Document) {
        self.1.borrow_mut().set_path(doc, path);
    }

    fn get(&self) -> Document {
        let local = self.1.borrow().clone();
        let parent = self.0.get();
        if local == Document::Unit {
            return parent;
        }
        parent + local
    }

    fn get_path(&self, path: &[&String]) -> Document {
        if path.is_empty() {
            return self.get();
        }
        let local = self.1.borrow().get_path(path).clone();
        let parent = self.0.get_path(path);
        if local == Document::Unit {
            return parent;
        }
        parent + local
    }
}
