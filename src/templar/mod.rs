use crate::*;

mod builder;
mod template;

pub use builder::TemplarBuilder;
pub use template::{Template, TemplateTree};

use std::collections::BTreeMap;

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
    pub fn parse<T: Parseable<U>, U>(&self, data: T) -> Result<U> {
        T::parse_into(data, self)
    }

    #[inline]
    #[cfg(feature = "json-extension")]
    pub fn parse_json(&self, json: &str) -> Result<TemplateTree> {
        Ok(self.parse(&serde_json::from_str(json).wrap()?)?)
    }

    #[inline]
    #[cfg(feature = "yaml-extension")]
    pub fn parse_yaml(&self, yml: &str) -> Result<TemplateTree> {
        Ok(self.parse(&serde_yaml::from_str(yml).wrap()?)?)
    }
}

pub trait Parseable<T>: private::Seal {
    fn parse_into(t: Self, templar: &Templar) -> Result<T>;
}

impl Parseable<Template> for &String {
    #[inline]
    fn parse_into(t: Self, templar: &Templar) -> Result<Template> {
        templar.parse_template(t)
    }
}

impl Parseable<Template> for &str {
    #[inline]
    fn parse_into(t: Self, templar: &Templar) -> Result<Template> {
        templar.parse_template(t)
    }
}

impl Parseable<Template> for &Document {
    #[inline]
    fn parse_into(t: Self, templar: &Templar) -> Result<Template> {
        Ok(match t {
            Document::String(s) => templar.parse_template(&s)?,
            Document::Newtype(d) => templar.parse(d.as_ref())?,
            _ => Node::Data(t.clone().into()).into(),
        })
    }
}

impl Parseable<TemplateTree> for &Document {
    #[inline]
    fn parse_into(doc: Self, templar: &Templar) -> Result<TemplateTree> {
        let default_context = Context::new_standard(Document::Unit);
        Ok(match doc {
            Document::Newtype(d) => templar.parse(d.as_ref())?,
            Document::Seq(s) => TemplateTree::Sequence(Arc::new(
                s.iter()
                    .map(|i| Ok(templar.parse(i)?))
                    .collect::<Result<Vec<TemplateTree>>>()?,
            )),
            Document::Map(map) => TemplateTree::Mapping(Arc::new(
                map.iter()
                    .map(|(k, v)| {
                        Ok((
                            templar.parse::<Self, Template>(k)?.exec(&default_context)?,
                            templar.parse(v)?,
                        ))
                    })
                    .collect::<Result<BTreeMap<Document, TemplateTree>>>()?,
            )),
            _ => TemplateTree::Template(templar.parse(doc)?),
        })
    }
}

mod private {
    pub trait Seal {}
    impl Seal for &unstructured::Document {}
    impl Seal for &String {}
    impl Seal for &str {}
}
