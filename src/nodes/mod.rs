use crate::*;
use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;
use unstructured::Document;

pub enum Node {
    Expr(Vec<Node>),
    Data(Document),
    Value(Vec<String>),
    Filter(Box<(Node, Arc<Filter>, Node)>),
    Method(Box<(Arc<Function>, Node)>),
    Array(Vec<Node>),
    Map(BTreeMap<Document, Node>),
    Empty(),
    Error(TemplarError),
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Filter(inner) => write!(f, "Node::Filter({:?} | {:?})", inner.0, inner.2),
            Node::Method(inner) => write!(f, "Node::Function({:?})", inner.1),
            Node::Empty() => write!(f, "Node::Empty()"),
            Node::Expr(inner) => write!(f, "Node::Expr({:?})", inner),
            Node::Data(inner) => write!(f, "Node::Data({:?})", inner),
            Node::Value(inner) => write!(f, "Node::Value({:?})", inner),
            Node::Array(inner) => write!(f, "Node::Array({:?})", inner),
            Node::Map(inner) => write!(f, "Node::Map({:?})", inner),
            Node::Error(inner) => write!(f, "Node::Error({:?})", inner),
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Node::Empty()
    }
}

impl Node {
    pub(crate) fn make_vector(self) -> Vec<Node> {
        match self {
            Self::Expr(inner) => inner,
            _ => vec![self],
        }
    }

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
            Self::Array(s) => {
                let mut elements = vec![];
                for node in s.iter() {
                    match node.exec(ctx) {
                        Self::Data(d) => elements.push(d),
                        error => return error,
                    };
                }
                Self::Data(Document::Seq(elements))
            }
            Self::Map(m) => {
                let mut map = BTreeMap::new();
                for (key, node) in m.iter() {
                    match node.exec(ctx) {
                        Self::Data(d) => map.insert(key.clone(), d),
                        error => return error,
                    };
                }
                Self::Data(map.into())
            }
            Self::Method(m) => {
                let (function, args) = (&m.0, &m.1);
                let a = args.exec(ctx).into_document();
                function(a).into()
            }
            Self::Empty() => Self::Data(Document::Unit),
            Self::Error(e) => Self::Error(e.clone()),
        }
    }

    pub(crate) fn into_document(self) -> Result<Document> {
        match self {
            Self::Data(d) => Ok(d),
            Self::Error(e) => Err(e),
            _ => Err(TemplarError::RenderFailure(
                "Attempted document conversion on unprocessed node".into(),
            )),
        }
    }

    pub fn render(&self, ctx: &dyn Context) -> Result<String> {
        match self {
            Node::Empty() => Ok("".into()),
            z => Ok(format!("{}", z.exec(ctx).into_document()?)),
        }
    }
}

impl From<Result<Document>> for Node {
    fn from(doc: Result<Document>) -> Node {
        match doc {
            Ok(d) => Self::Data(d),
            Err(e) => Self::Error(e),
        }
    }
}

impl From<Vec<Node>> for Node {
    fn from(mut n: Vec<Node>) -> Node {
        match n.len() {
            1 => n.pop().unwrap(),
            0 => Node::Empty(),
            _ => Node::Expr(n),
        }
    }
}
