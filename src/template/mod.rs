use crate::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Template {
    Node(Arc<Node>),
    Mapping(Arc<HashMap<Document, Template>>),
    Sequence(Arc<Vec<Template>>),
}

impl Template {
    pub fn get(&self, key: &Document) -> Result<Template> {
        match self {
            Template::Mapping(v) => Ok(v[key].clone()),
            _ => Err(TemplarError::RenderFailure(
                "Attempted to map into var that is not a mapping".into(),
            )),
        }
    }

    pub fn get_path<T: Into<Document>>(&self, path: Vec<T>) -> Result<Template> {
        let mut result = self.clone();
        for key in path.into_iter() {
            result = result.get(&key.into())?;
        }
        Ok(result)
    }

    pub fn get_index(&self, index: usize) -> Result<Template> {
        match self {
            Template::Sequence(v) => Ok(v[index].clone()),
            _ => Err(TemplarError::RenderFailure(
                "Attempted to index into non-sequence".into(),
            )),
        }
    }

    pub fn render(&self, ctx: &dyn Context) -> Result<String> {
        match self {
            Template::Node(n) => Ok(n.render(ctx)?),
            Template::Mapping(..) => Err(TemplarError::RenderFailure(
                "Cannot render a mapping directly!".into(),
            )),
            Template::Sequence(..) => Err(TemplarError::RenderFailure(
                "Cannot render a sequence directly!".into(),
            )),
        }
    }

    pub fn exec(&self, ctx: &dyn Context) -> Result<Document> {
        match self {
            Template::Node(n) => n.exec(ctx).into_document(),
            _ => Err(TemplarError::RenderFailure(
                "Template not executable!".into(),
            )),
        }
    }
}

impl From<Node> for Template {
    fn from(n: Node) -> Template {
        Template::Node(Arc::new(n))
    }
}
