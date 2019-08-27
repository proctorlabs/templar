use crate::*;

mod builder;

pub use builder::TemplarBuilder;

lazy_static! {
    static ref GLOBAL: Templar = { Templar::default() };
}

pub struct Templar {
    pub(crate) functions: HashMap<String, Arc<Function>>,
    pub(crate) filters: HashMap<String, Arc<Filter>>,
}

impl Default for Templar {
    fn default() -> Templar {
        TemplarBuilder::default().build()
    }
}

impl Templar {
    #[inline]
    pub fn global() -> &'static Templar {
        &GLOBAL
    }

    #[inline]
    pub fn parse_tree(&self, doc: &Document) -> Result<TemplateTree> {
        let default_context = StandardContext::new(Document::Unit);
        Ok(match doc {
            Document::Newtype(d) => self.parse_tree(d)?,
            Document::Seq(s) => TemplateTree::Sequence(Arc::new(
                s.iter()
                    .map(|i| Ok(self.parse_tree(i)?))
                    .collect::<Result<Vec<TemplateTree>>>()?,
            )),
            Document::Map(map) => TemplateTree::Mapping(Arc::new(
                map.iter()
                    .map(|(k, v)| Ok((self.parse(k)?.exec(&default_context)?, self.parse_tree(v)?)))
                    .collect::<Result<HashMap<Document, TemplateTree>>>()?,
            )),
            _ => TemplateTree::Template(self.parse(doc)?),
        })
    }

    #[inline]
    pub fn parse(&self, doc: &Document) -> Result<Template> {
        Ok(match doc {
            Document::String(s) => self.parse_template(&s)?,
            Document::Newtype(d) => self.parse(d)?,
            _ => Node::Data(doc.clone()).into(),
        })
    }

    #[inline]
    pub fn parse_template(&self, val: &str) -> Result<Template> {
        self.parse_text(val, true)
    }

    #[inline]
    pub fn parse_expression(&self, val: &str) -> Result<Template> {
        self.parse_text(val, false)
    }

    #[inline]
    pub fn parse_json(&self, json: &str) -> Result<TemplateTree> {
        Ok(self.parse_tree(&serde_json::from_str(json)?)?)
    }

    #[inline]
    pub fn parse_yaml(&self, yml: &str) -> Result<TemplateTree> {
        Ok(self.parse_tree(&serde_yaml::from_str(yml)?)?)
    }
}
