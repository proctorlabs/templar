use crate::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Template(Arc<Vec<Node>>);

impl Template {
    pub fn render(&self, ctx: &Document) -> Result<String> {
        let mut result = String::new();
        for node in self.0.iter() {
            result.push_str(&node.render(&ctx)?);
        }
        Ok(result)
    }
}

impl From<Vec<Node>> for Template {
    fn from(nodes: Vec<Node>) -> Template {
        Template(Arc::new(nodes))
    }
}
