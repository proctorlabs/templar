use super::*;

use {parking_lot::RwLock, std::sync::Arc};

/// This context type can be shared between threads safely
#[derive(Debug, Clone)]
pub struct StandardContext(Arc<RwLock<ContextMap>>);

impl Default for StandardContext {
    fn default() -> Self {
        Self::new()
    }
}

impl StandardContext {
    /// Create a new empty shared context
    pub fn new() -> Self {
        StandardContext(Arc::new(RwLock::new(ContextMap::new(InnerData::Null))))
    }
}

impl Context for StandardContext {
    fn set_path_inner(&self, path: &[&InnerData], doc: ContextMapValue) -> Result<()> {
        self.0.write().set(doc, path)
    }

    fn get_path_inner(&self, path: &[&InnerData], ctx: &impl Context) -> Data {
        self.0.read().exec(ctx, path)
    }

    fn wrap(&self) -> ContextWrapper {
        ContextWrapper::Standard(self)
    }
}
