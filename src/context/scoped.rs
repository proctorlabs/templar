pub use super::*;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Debug)]
#[allow(clippy::borrowed_box)]
pub struct ScopedContext<'a>(&'a Box<dyn ContextDispatcher + 'a>, Rc<RefCell<ContextMap>>);

#[allow(clippy::borrowed_box)]
impl<'a> ScopedContext<'a> {
    pub fn new(ctx: &'a Box<dyn ContextDispatcher + 'a>) -> Self {
        ScopedContext(ctx, Default::default())
    }
}

impl<'a> ContextDispatcher for ScopedContext<'a> {
    fn set_path(&self, path: &[&Document], doc: ContextMapValue) -> Result<()> {
        self.1.borrow_mut().set(doc, path)
    }

    fn get_path(&self, path: &[&Document], ctx: &Context) -> Data {
        let local = self.1.borrow().exec(ctx, path);
        if local.is_empty() {
            self.0.get_path(path, ctx)
        } else {
            local
        }
    }
}
