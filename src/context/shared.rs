use super::*;

use {parking_lot::RwLock, std::sync::Arc};

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
        if path.is_empty() {
            return self.get();
        }
        self.0.read().get_path(path).clone()
    }
}
