use super::*;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

/// A single-threaded context
#[derive(Debug, Clone)]
pub struct StandardContext(Rc<RefCell<ContextMap>>);

impl Default for StandardContext {
    fn default() -> Self {
        Self::new()
    }
}

impl StandardContext {
    /// Create a new empty standard context
    pub fn new() -> Self {
        StandardContext(Rc::new(RefCell::new(ContextMap::new(Document::Null))))
    }
}

impl Context for StandardContext {
    fn set_path_inner(&self, path: &[&Document], doc: ContextMapValue) -> Result<()> {
        self.0.borrow_mut().set(doc, path)
    }

    fn get_path_inner(&self, path: &[&Document], ctx: &impl Context) -> Data {
        self.0.borrow().exec(ctx, path)
    }

    fn wrap(&self) -> ContextWrapper {
        ContextWrapper::Standard(self)
    }
}
