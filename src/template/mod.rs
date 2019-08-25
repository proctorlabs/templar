use crate::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Template(Arc<Node>);

impl Template {
    pub fn render(&self, ctx: &Document) -> Result<String> {
        Ok(self.0.render(&ctx)?)
    }
}

impl From<Node> for Template {
    fn from(nodes: Node) -> Template {
        Template(Arc::new(nodes))
    }
}
