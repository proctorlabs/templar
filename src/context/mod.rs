use parking_lot::RwLock;
use std::{cell::RefCell, rc::Rc, sync::Arc};
use unstructured::Document;

pub trait Context {
    fn merge(&self, doc: Document);

    fn set(&self, doc: Document);

    fn set_path(&self, path: &[&String], doc: Document);

    fn get(&self) -> Document;

    fn get_path(&self, path: &[&String]) -> Document;
}

#[derive(Debug, Clone)]
pub struct SharedContext(Arc<RwLock<Document>>);

impl SharedContext {
    pub fn new(initial_value: Document) -> Self {
        SharedContext(Arc::new(RwLock::new(initial_value)))
    }
}

impl Context for SharedContext {
    fn merge(&self, doc: Document) {
        self.0.write().merge(doc);
    }

    fn set(&self, doc: Document) {
        self.0.write().set(doc);
    }

    fn set_path(&self, path: &[&String], doc: Document) {
        self.0.write().set_path(doc, path);
    }

    fn get(&self) -> Document {
        self.0.read().clone()
    }

    fn get_path(&self, path: &[&String]) -> Document {
        self.0.read().get_path(path).clone()
    }
}

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
        self.0.borrow().get_path(path).clone()
    }
}
