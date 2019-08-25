use crate::*;
use std::sync::Arc;
use unstructured::Document;

#[derive(Debug)]
pub enum Node {
    Expr(Vec<Node>),
    Data(Document),
    Value(Vec<String>),
    Filter(Box<(Node, Arc<Filter>, Node)>),
    Method(Box<(Arc<Function>, Node)>),
    Empty(),
    Error(String),
}

impl Node {
    pub(crate) fn exec(&self, ctx: &dyn Context) -> Node {
        match self {
            Self::Data(d) => Self::Data(d.clone()),
            Self::Expr(a) => {
                let mut res: Vec<Document> = vec![];
                for node in a.iter() {
                    match node.exec(ctx) {
                        Self::Data(d) => res.push(d),
                        error => return error,
                    };
                }
                if res.is_empty() {
                    Self::Data(Document::Unit)
                } else if res.len() == 1 {
                    Self::Data(res.pop().unwrap())
                } else {
                    Self::Data(res.into())
                }
            }
            Self::Value(a) => {
                Self::Data(ctx.get_path(&a.iter().map(|a| a).collect::<Vec<&String>>()))
            }
            Self::Filter(b) => {
                let (piped, filter, args) = (&b.0, &b.1, &b.2);
                let p = piped.exec(ctx).into_document();
                let a = args.exec(ctx).into_document();
                filter(p, a).into()
            }
            Self::Method(m) => {
                let (function, args) = (&m.0, &m.1);
                let a = args.exec(ctx).into_document();
                function(a).into()
            }
            Self::Empty() => Self::Data(Document::Unit),
            Self::Error(e) => Self::Error(e.to_owned()),
        }
    }

    pub(crate) fn into_document(self) -> Result<Document> {
        match self {
            Self::Data(d) => Ok(d),
            Self::Error(s) => Err(TemplarError::RenderFailure(s)),
            _ => Err(TemplarError::RenderFailure(
                "Attempted document conversion on unprocessed node".into(),
            )),
        }
    }

    pub fn render(&self, ctx: &dyn Context) -> Result<String> {
        Ok(format!("{}", self.exec(ctx).into_document()?))
    }
}

impl From<Result<Document>> for Node {
    fn from(doc: Result<Document>) -> Node {
        match doc {
            Ok(d) => Self::Data(d),
            Err(e) => Self::Error(format!("{:?}", e)),
        }
    }
}
