use super::*;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Debug, Clone)]
pub struct StandardContext(Rc<RefCell<ContextMap>>);

impl StandardContext {
    pub fn new(initial_value: Document) -> Self {
        StandardContext(Rc::new(RefCell::new(ContextMap::new(initial_value))))
    }
}

impl Context for StandardContext {
    fn set_path(&self, path: &[Document], doc: Document) {
        self.0.borrow_mut().set(doc, path).unwrap_or_default();
    }

    fn get_path(&self, path: &[Document]) -> Document {
        self.0
            .borrow()
            .exec(self, path)
            .result()
            .unwrap_or_default()
    }
}
