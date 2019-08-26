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
    pub fn parse(&self, doc: &Document) -> Result<Template> {
        let default_context = StandardContext::new(Document::Unit);
        Ok(match doc {
            Document::String(s) => self.parse_expression(&s)?,
            Document::Newtype(d) => self.parse(d)?,
            Document::Seq(s) => Template::Sequence(Arc::new(
                s.iter()
                    .map(|i| Ok(self.parse(i)?))
                    .collect::<Result<Vec<Template>>>()?,
            )),
            Document::Map(map) => Template::Mapping(Arc::new(
                map.iter()
                    .map(|(k, v)| Ok((self.parse(k)?.exec(&default_context)?, self.parse(v)?)))
                    .collect::<Result<HashMap<Document, Template>>>()?,
            )),
            _ => Template::Node(Arc::new(Node::Data(doc.clone()))),
        })
    }

    #[inline]
    pub fn parse_expression(&self, val: &str) -> Result<Template> {
        self.parse_template(val)
    }

    #[inline]
    pub fn parse_json(&self, json: &str) -> Result<Template> {
        Ok(self.parse(&serde_json::from_str::<Document>(json).unwrap())?)
    }

    #[inline]
    pub fn parse_yaml(&self, yml: &str) -> Result<Template> {
        Ok(self.parse(&serde_yaml::from_str::<Document>(yml).unwrap())?)
    }
}
