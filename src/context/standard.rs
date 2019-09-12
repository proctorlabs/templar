use super::*;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Debug, Clone)]
pub struct StandardContext(Rc<RefCell<ContextMap>>);

impl StandardContext {
    pub fn new<T: Into<ContextMapValue>>(initial_value: T) -> Self {
        StandardContext(Rc::new(RefCell::new(ContextMap::new(initial_value))))
    }
}

impl ContextDispatcher for StandardContext {
    fn set_path(&self, path: &[&Document], doc: ContextMapValue) -> Result<()> {
        self.0.borrow_mut().set(doc, path)
    }

    fn get_path(&self, path: &[&Document], ctx: &Context) -> Data {
        self.0.borrow().exec(ctx, path)
    }
}
