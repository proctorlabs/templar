use super::*;

use {parking_lot::RwLock, std::sync::Arc};

#[derive(Debug, Clone)]
pub struct SharedContext(Arc<RwLock<ContextMap>>);

impl SharedContext {
    pub fn new(initial_value: Document) -> Self {
        SharedContext(Arc::new(RwLock::new(ContextMap::new(initial_value))))
    }
}

impl Context for SharedContext {
    fn set_path(&self, path: &[Document], doc: Document) {
        self.0.write().set(doc, path).unwrap_or_default();
    }

    fn get_path(&self, path: &[Document]) -> Document {
        self.0.read().exec(self, path).result().unwrap_or_default()
    }
}
