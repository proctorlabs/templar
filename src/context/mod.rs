use parking_lot::RwLock;
use std::{cell::RefCell, rc::Rc, sync::Arc};
use unstructured::Document;

#[derive(Debug, Clone)]
pub struct StandardContext(Rc<RefCell<Document>>);

#[derive(Debug, Clone)]
pub struct SharedContext(Arc<RwLock<Document>>);

pub trait Context {
    fn merge(&self, doc: Document);

    fn set_path(&self, path: &[&String], doc: Document);

    fn get_path(&self, path: &[&String]) -> Document;
}

impl Context for SharedContext {
    fn merge(&self, doc: Document) {
        self.0.write().merge(doc);
    }

    fn set_path(&self, path: &[&String], doc: Document) {
        self.0.write().set_path(doc, path);
    }

    fn get_path(&self, path: &[&String]) -> Document {
        self.0.read().get_path(path).clone()
    }
}

impl Context for StandardContext {
    fn merge(&self, doc: Document) {
        self.0.borrow_mut().merge(doc);
    }

    fn set_path(&self, path: &[&String], doc: Document) {
        self.0.borrow_mut().set_path(doc, path);
    }

    fn get_path(&self, path: &[&String]) -> Document {
        self.0.borrow().get_path(path).clone()
    }
}

impl StandardContext {
    pub fn new(initial_value: Document) -> Self {
        StandardContext(Rc::new(RefCell::new(initial_value)))
    }
}

// Disabling for now as I would like both contexts to have the same interface
// std::ops::Deref
// impl Deref for StandardContext {
//     type Target = Document;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
