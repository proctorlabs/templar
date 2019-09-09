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
    fn set_path(&self, path: &[Document], doc: ContextMapValue) -> Result<()> {
        self.1.borrow_mut().set(doc, path)
    }

    fn get_path(&self, path: &[Document]) -> Data {
        let local = self.1.borrow().exec(self, path);
        let parent = self.0.get_path(path);
        if local.is_empty() {
            parent
        } else if (local.is_empty() && parent.is_empty()) || local.is_failed() || parent.is_failed()
        {
            local
        } else {
            Data::from(parent.result().unwrap_or_default() + local.result().unwrap_or_default())
        }
    }
}
