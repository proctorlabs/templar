use super::*;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Debug, Clone)]
pub struct StandardContext(Rc<RefCell<Document>>);

impl StandardContext {
    pub fn new(initial_value: Document) -> Self {
        StandardContext(Rc::new(RefCell::new(initial_value)))
    }
}

impl Context for StandardContext {
    fn merge(&self, doc: Document) {
        self.0.borrow_mut().merge(doc);
    }

    fn set(&self, doc: Document) {
        self.0.borrow_mut().set(doc);
    }

    fn set_path(&self, path: &[&String], doc: Document) {
        self.0.borrow_mut().set_path(doc, path);
    }

    fn get(&self) -> Document {
        self.0.borrow().clone()
    }

    fn get_path(&self, path: &[&String]) -> Document {
        if path.is_empty() {
            return self.get();
        }
        self.0.borrow().get_path(path).clone()
    }
}
