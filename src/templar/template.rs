use crate::context::ScopedContext;
use crate::*;
use std::convert::TryFrom;
use std::sync::Arc;

#[derive(Debug, Clone, Default)]
pub struct Template(Arc<Node>);

impl Template {
    pub fn render(&self, ctx: &dyn Context) -> Result<String> {
        let local_ctx = ScopedContext::new(ctx);
        self.0.render(&local_ctx)
    }

    pub fn exec(&self, ctx: &dyn Context) -> Result<Document> {
        self.0.exec(ctx).result()
    }

    pub(crate) fn root_node(&self) -> Arc<Node> {
        self.0.clone()
    }
}

#[derive(Debug, Clone)]
pub enum TemplateTree {
    Template(Template),
    Mapping(Arc<HashMap<Document, TemplateTree>>),
    Sequence(Arc<Vec<TemplateTree>>),
}

impl Default for TemplateTree {
    fn default() -> Self {
        TemplateTree::Template(Default::default())
    }
}

impl TemplateTree {
    pub fn get(&self, key: &Document) -> Option<TemplateTree> {
        match self {
            TemplateTree::Mapping(v) => Some(v[key].clone()),
            _ => None,
        }
    }

    pub fn get_path<T: Into<Document> + Clone>(&self, path: &[T]) -> Option<TemplateTree> {
        let mut result = self.clone();
        for key in path.iter() {
            result = result.get(&key.clone().into())?;
        }
        Some(result)
    }

    pub fn get_index(&self, index: usize) -> Option<TemplateTree> {
        match self {
            TemplateTree::Sequence(v) => Some(v.get(index).cloned().unwrap_or_default()),
            _ => None,
        }
    }
}

impl From<Node> for Template {
    fn from(n: Node) -> Template {
        Template(Arc::new(n))
    }
}

impl From<Template> for TemplateTree {
    fn from(t: Template) -> TemplateTree {
        TemplateTree::Template(t)
    }
}

impl TryFrom<TemplateTree> for Template {
    type Error = TemplarError;

    fn try_from(value: TemplateTree) -> Result<Self> {
        match value {
            TemplateTree::Template(t) => Ok(t),
            _ => Err(TemplarError::ParseFailure("Not a template node".into())),
        }
    }
}

impl TryFrom<Option<TemplateTree>> for Template {
    type Error = TemplarError;

    fn try_from(value: Option<TemplateTree>) -> Result<Self> {
        match value {
            Some(TemplateTree::Template(t)) => Ok(t),
            _ => Err(TemplarError::ParseFailure("Not a template node".into())),
        }
    }
}
