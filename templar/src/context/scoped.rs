pub use super::*;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Debug)]
#[allow(clippy::borrowed_box)]
pub struct ScopedContext<'a>(ContextWrapper<'a>, Rc<RefCell<ContextMap>>);

#[allow(clippy::borrowed_box)]
impl<'a> ScopedContext<'a> {
    pub fn new(ctx: ContextWrapper<'a>) -> Self {
        ScopedContext(ctx, Default::default())
    }
}

impl<'a> Context for ScopedContext<'a> {
    fn set_path_inner(&self, path: &[&InnerData], doc: ContextMapValue) -> Result<()> {
        self.1.borrow_mut().set(doc, path)
    }

    fn get_path_inner(&self, path: &[&InnerData], ctx: &impl Context) -> Data {
        let local = self.1.borrow().exec(ctx, path);
        if local.is_empty() {
            self.0.get_path_inner(path, ctx)
        } else {
            local
        }
    }

    fn wrap(&self) -> ContextWrapper {
        ContextWrapper::Scope(self)
    }
}
