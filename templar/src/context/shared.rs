use super::*;

use {parking_lot::RwLock, std::sync::Arc};

/// This context type can be shared between threads safely
#[derive(Debug, Clone)]
pub struct SharedContext(Arc<RwLock<ContextMap>>);

impl Default for SharedContext {
    fn default() -> Self {
        Self::new()
    }
}

impl SharedContext {
    /// Create a new empty shared context
    pub fn new() -> Self {
        SharedContext(Arc::new(RwLock::new(ContextMap::new(InnerData::Null))))
    }
}

impl Context for SharedContext {
    fn set_path_inner(&self, path: &[&InnerData], doc: ContextMapValue) -> Result<()> {
        self.0.write().set(doc, path)
    }

    fn get_path_inner(&self, path: &[&InnerData], ctx: &impl Context) -> Data {
        self.0.read().exec(ctx, path)
    }

    fn wrap(&self) -> ContextWrapper {
        ContextWrapper::Shared(self)
    }
}
