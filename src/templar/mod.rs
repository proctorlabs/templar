/*!
The Templar module
 */

use crate::*;

mod builder;
mod template;

pub use builder::TemplarBuilder;
pub use template::{Template, TemplateTree};

use std::collections::BTreeMap;

lazy_static! {
    static ref GLOBAL: Templar = { Templar::default() };
}

/// The Templar struct is the primary template parser.
///
/// A new customized instance can be created using TemplarBuilder or alternatively
/// Templar::global() can be used if the default configurations are acceptable.
///
/// # Usage
///
/// ```
/// use templar::{Templar, Context};
/// use unstructured::Document;
///
/// let context = Context::new_standard(Document::Unit);
///
/// // parse and render a template, render returns a string
/// let template = Templar::global().parse("This is a template with {{ 'an expression' }}")?;
/// assert_eq!(template.render(&context)?, "This is a template with an expression");
///
/// // parse and execute an expression, this can be converted to most native types
/// let expression = Templar::global().parse_expression("5 + 5")?;
/// assert_eq!(expression.exec(&context)?, 10 as i64);
/// # Ok::<(), templar::TemplarError>(())
/// ```
pub struct Templar {
    pub(crate) functions: HashMap<String, Arc<functions::Function>>,
    pub(crate) filters: HashMap<String, Arc<filters::Filter>>,
}

impl Default for Templar {
    fn default() -> Templar {
        TemplarBuilder::default().build()
    }
}

impl Templar {
    /// Retrieve the global default instance of Templar when the defaults meet your needs.
    #[inline]
    pub fn global() -> &'static Templar {
        &GLOBAL
    }

    /// Parse a `Template` or `TemplateTree` value.
    ///
    /// ```
    /// # use templar::{Templar, Context, Template};
    /// # use unstructured::Document;
    /// # use std::convert::TryInto;
    ///
    /// # let context = Context::new_standard(Document::Unit);
    ///
    /// let template: Template = Templar::global().parse("{{ [5, 8, 3] | index(0) }}")?;
    /// assert_eq!(template.exec(&context)?, 5 as i64);
    /// # Ok::<(), templar::TemplarError>(())
    /// ```
    #[inline]
    pub fn parse<T: Parseable<U>, U>(&self, data: T) -> Result<U> {
        T::parse_into(data, self)
    }

    /// Parse a JSON string to a TemplateTree. This is useful if you want to parse a configuration
    /// file directly to a context as TemplateTree is directly convertible to a context.
    ///
    /// # Usage
    ///
    /// ```
    /// # use templar::{Templar, Context, Template};
    /// # use unstructured::Document;
    /// # use std::convert::TryInto;
    ///
    /// let json_string = r#"
    /// {
    ///     "key": "{{ script('echo -n test') | key('stdout') }}"
    /// }
    /// "#;
    ///
    /// # let context = Context::new_standard(Document::Unit);
    ///
    /// let tree = Templar::global().parse_json(json_string)?;
    /// let template: Template = tree.get_path(&["key"]).try_into()?;
    ///
    /// assert_eq!(template.render(&context)?, "test");
    /// # Ok::<(), templar::TemplarError>(())
    /// ```
    #[inline]
    #[cfg(feature = "json-extension")]
    pub fn parse_json(&self, json: &str) -> Result<TemplateTree> {
        Ok(self.parse(&serde_json::from_str(json).wrap()?)?)
    }

    /// Identical to parse_json except this expects a YAML string.
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
