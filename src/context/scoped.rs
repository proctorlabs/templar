pub use super::*;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Debug, Clone)]
pub(crate) struct ScopedContext<'c>(&'c dyn Context, Rc<RefCell<ContextMap>>);

impl<'c> ScopedContext<'c> {
    pub fn new(ctx: &'c dyn Context) -> Self {
        ScopedContext(ctx, Default::default())
    }
}

impl<'c> Context for ScopedContext<'c> {
    fn set_path(&self, path: &[Document], doc: Document) {
        self.1.borrow_mut().set(doc, path).unwrap_or_default();
    }

    fn get_path(&self, path: &[Document]) -> Document {
        let local = self
            .1
            .borrow()
            .exec(self, path)
            .new_result()
            .unwrap_or_default();
        let parent = self.0.get_path(path);
        if local.is_unit() || local == Document::from("") {
            parent
        } else {
            parent + local
        }
    }
}
