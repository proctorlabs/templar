use super::*;

#[derive(Clone)]
pub enum Node {
    Expr(Vec<Node>),
    Data(Data),
    Scope(Box<Node>),
    Value(Vec<String>),
    Operation(Arc<Operation>),
    Filter(Box<(Node, Arc<Filter>, Node)>),
    Function(Box<(Arc<Function>, Node)>),
    Array(Vec<Node>),
    Map(BTreeMap<Document, Node>),
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Filter(inner) => write!(f, "Node::Filter({:?} | {:?})", inner.0, inner.2),
            Node::Function(inner) => write!(f, "Node::Function({:?})", inner.1),
            Node::Expr(inner) => write!(f, "Node::Expr({:?})", inner),
            Node::Operation(inner) => write!(f, "Node::Operation({:?})", inner),
            Node::Data(inner) => write!(f, "Node::Data({:?})", inner),
            Node::Value(inner) => write!(f, "Node::Value({:?})", inner),
            Node::Array(inner) => write!(f, "Node::Array({:?})", inner),
            Node::Map(inner) => write!(f, "Node::Map({:?})", inner),
            Node::Scope(inner) => write!(f, "Node::Scope({:?})", inner),
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Data::empty().into()
    }
}

impl Node {
    pub(crate) fn exec(&self, ctx: &Context) -> Data {
        match self {
            Self::Data(d) => d.clone(),
            Self::Expr(a) => {
                let mut res: Vec<Data> = a.iter().map(|n| n.exec(ctx)).collect();
                if res.is_empty() {
                    Data::empty()
                } else if res.len() == 1 {
                    res.pop().unwrap()
                } else {
                    Data::from_vec(res)
                }
            }
            Self::Value(a) => {
                let docs = a.iter().map(|a| a.into()).collect::<Vec<Document>>();
                ctx.get_path(&docs)
            }
            Self::Operation(op) => op.exec(ctx),
            Self::Filter(b) => {
                let (piped, filter, args) = (&b.0, &b.1, &b.2);
                let p = piped.exec(ctx).result();
                let a = args.exec(ctx).result();
                match filter(p, a) {
                    Ok(d) => d.into(),
                    Err(e) => e.into(),
                }
            }
            Self::Scope(i) => {
                let local_context = ctx.create_scope();
                i.exec(&local_context)
            }
            Self::Array(s) => Data::from_vec(s.iter().map(|n| n.exec(ctx)).collect()),
            Self::Map(m) => {
                let mut map: BTreeMap<Document, Document> = BTreeMap::new();
                for (key, node) in m.iter() {
                    match node.exec(ctx).result() {
                        Ok(d) => map.insert(key.clone(), d),
                        Err(e) => return e.into(),
                    };
                }
                map.into()
            }
            Self::Function(m) => {
                let (function, args) = (&m.0, &m.1);
                let a = args.exec(ctx).result();
                match function(a) {
                    Ok(d) => d.into(),
                    Err(e) => e.into(),
                }
            }
        }
    }

    pub(crate) fn set_operation(self, op: Operations) -> Node {
        match self {
            Node::Expr(nodes) => Node::Operation(Arc::new(op.build(nodes))),
            _ => self,
        }
    }

    pub(crate) fn into_scope(self) -> Node {
        Node::Scope(Box::new(self))
    }

    pub(crate) fn into_document(self) -> Result<Document> {
        match self {
            Self::Data(d) => Ok(d.result()?),
            _ => Err(TemplarError::RenderFailure(
                "Attempted document conversion on unprocessed node".into(),
            )),
        }
    }

    pub fn render(&self, ctx: &Context) -> Result<String> {
        self.exec(ctx).render()
    }
}

impl From<Vec<Node>> for Node {
    fn from(mut n: Vec<Node>) -> Node {
        match n.len() {
            1 => n.pop().unwrap(),
            0 => Data::empty().into(),
            _ => Node::Expr(n),
        }
    }
}

impl From<Data> for Node {
    fn from(d: Data) -> Self {
        Self::Data(d)
    }
}
