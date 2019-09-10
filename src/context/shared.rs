use super::*;

use {parking_lot::RwLock, std::sync::Arc};

#[derive(Debug, Clone)]
pub struct SharedContext(Arc<RwLock<ContextMap>>);

impl SharedContext {
    pub fn new<T: Into<ContextMapValue>>(initial_value: T) -> Self {
        SharedContext(Arc::new(RwLock::new(ContextMap::new(initial_value))))
    }
}

impl ContextDispatcher for SharedContext {
    fn set_path(&self, path: &[Document], doc: ContextMapValue) -> Result<()> {
        self.0.write().set(doc, path)
    }

    fn get_path(&self, path: &[Document], ctx: &Context) -> Data {
        self.0.read().exec(ctx, path)
    }
}
