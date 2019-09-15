use crate::*;
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::sync::Arc;

/// Template holds the prepared result of parsing a template. Because the template does not need to be
/// reparsed, subsequent executions come at a low cost.
#[derive(Debug, Clone, Default)]
pub struct Template(Arc<Node>);

impl Template {
    /// Render a template as a string.
    ///
    /// # Usage
    ///
    /// ```
    /// # use templar::*;
    /// # use unstructured::Document;
    /// # let context = Context::new_standard(Document::Unit);
    ///
    /// let t = Templar::global().parse_expression("5 + 5")?;
    /// assert_eq!(t.render(&context)?, "10");
    /// # Ok::<(), templar::TemplarError>(())
    /// ```
    pub fn render(&self, ctx: &Context) -> Result<String> {
        let local_ctx = ctx.create_scope();
        self.0.render(&local_ctx)
    }

    /// Execute a template, getting a `Document` from the `unstructured` crate as a result.
    /// many of the native rust types implement into() on Document making direct comparisons
    /// easy.
    ///
    /// # Usage
    ///
    /// ```
    /// # use templar::*;
    /// # use unstructured::Document;
    /// # let context = Context::new_standard(Document::Unit);
    ///
    /// let t = Templar::global().parse_expression("5 + 5")?;
    /// assert_eq!(t.exec(&context)?, 10i64);
    /// # Ok::<(), templar::TemplarError>(())
    /// ```
    pub fn exec(&self, ctx: &Context) -> Result<Document> {
        let local_ctx = ctx.create_scope();
        self.0.exec(&local_ctx).into_result()
    }

    pub(crate) fn root_node(&self) -> Arc<Node> {
        self.0.clone()
    }
}

/// TemplateTree holds the parsed result of a Document tree. This tree of templates
/// can then be loaded directly into a context.
#[derive(Debug, Clone)]
pub enum TemplateTree {
    /// This TemplateTree node contains a template. This node type can be converted into a `Template`
    Template(Template),
    /// This TemplateTree node contains a mapping
    Mapping(Arc<BTreeMap<Document, TemplateTree>>),
    /// This TemplateTree node contains a sequence
    Sequence(Arc<Vec<TemplateTree>>),
}

impl Default for TemplateTree {
    fn default() -> Self {
        TemplateTree::Template(Default::default())
    }
}

impl TemplateTree {
    /// Attempt to walk the tree with the specified key
    pub fn get(&self, key: &Document) -> Option<TemplateTree> {
        match self {
            TemplateTree::Mapping(v) => Some(v[key].clone()),
            _ => None,
        }
    }

    /// Attempt to walk the tree with the specified path
    pub fn get_path<T: Into<Document> + Clone>(&self, path: &[T]) -> Option<TemplateTree> {
        let mut result = self.clone();
        for key in path.iter() {
            result = result.get(&key.clone().into())?;
        }
        Some(result)
    }

    /// Attempt to retrieve an index from a sequence of the tree
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
