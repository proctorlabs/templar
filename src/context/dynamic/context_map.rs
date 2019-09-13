use super::*;

#[derive(Clone, Debug, Default)]
pub struct ContextMap {
    root: BTreeMap<Document, ContextMapValue>,
}

impl ContextMap {
    pub fn new<T: Into<ContextMapValue>>(doc: T) -> Self {
        let mut result = ContextMap::default();
        result.set(doc, &[]).unwrap_or_default();
        result
    }

    pub fn set<T: Into<ContextMapValue>>(&mut self, value: T, path: &[&Document]) -> Result<()> {
        if path.is_empty() {
            let val: ContextMapValue = value.into();
            if let ContextMapValue::Map(map) = val {
                for (k, v) in map.into_iter() {
                    self.root.insert(k, v);
                }
            }
            return Ok(());
        }
        if path.len() == 1 {
            self.root.insert(path[0].clone(), value.into());
            return Ok(());
        }
        let mut target: &mut ContextMapValue = self
            .root
            .entry(path[0].clone())
            .or_insert_with(ContextMapValue::new_map);
        for p in path.iter().skip(1).take(path.len() - 1) {
            target = target.get_or_add_key(*p);
        }
        target.set(value.into());
        Ok(())
    }

    pub fn exec(&self, ctx: &Context, path: &[&Document]) -> Data {
        if path.is_empty() {
            let copy = ContextMapValue::Map(self.root.clone());
            return copy.exec(ctx);
        }
        let walker = ContextWalk::from(self.root.get(&path[0]));
        for p in path.iter().skip(1) {
            walker.walk(ctx, p);
        }
        walker.exec(ctx)
    }
}

impl From<TemplateTree> for ContextMapValue {
    fn from(val: TemplateTree) -> Self {
        match val {
            TemplateTree::Template(t) => t.into(),
            TemplateTree::Sequence(s) => {
                let result: Vec<ContextMapValue> = s.iter().map(|t| t.clone().into()).collect();
                ContextMapValue::Seq(result)
            }
            TemplateTree::Mapping(m) => {
                let result: BTreeMap<Document, ContextMapValue> = m
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone().into()))
                    .collect();
                ContextMapValue::Map(result)
            }
        }
    }
}

impl ContextMapValue {
    #[inline]
    fn new_map() -> Self {
        ContextMapValue::Map(BTreeMap::new())
    }

    fn set<T: Into<ContextMapValue>>(&mut self, val: T) {
        replace(self, val.into());
    }

    fn get_or_add_key(&mut self, key: &Document) -> &mut ContextMapValue {
        match self {
            ContextMapValue::Map(ref mut map) => map
                .entry(key.clone())
                .or_insert_with(ContextMapValue::new_map),
            _ => {
                let new_val = ContextMapValue::new_map();
                replace(self, new_val);
                self.get_or_add_key(key)
            }
        }
    }

    pub fn exec(&self, ctx: &Context) -> Data {
        match self {
            ContextMapValue::Node(node) => node.exec(ctx),
            ContextMapValue::Map(map) => {
                let mut result: BTreeMap<Document, Document> = BTreeMap::new();
                for (k, v) in map.iter() {
                    match v.exec(ctx).result() {
                        Ok(d) => result.insert(k.clone(), d),
                        Err(e) => return e.into(),
                    };
                }
                result.into()
            }
            ContextMapValue::Seq(s) => {
                let result: Result<Vec<Document>> =
                    s.iter().map(|v| v.exec(ctx).result()).collect();
                match result {
                    Ok(s) => s.into(),
                    Err(e) => e.into(),
                }
            }
            _ => Data::empty(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ContextMapValue {
    Seq(Vec<ContextMapValue>),
    Map(BTreeMap<Document, ContextMapValue>),
    Node(Arc<Node>),
    Empty,
}

impl Default for ContextMapValue {
    fn default() -> Self {
        ContextMapValue::Empty
    }
}

impl<T: Into<Document>> From<T> for ContextMapValue {
    fn from(val: T) -> Self {
        match val.into() {
            Document::Map(m) => {
                let mut new_val = BTreeMap::new();
                for (k, v) in m.into_iter() {
                    new_val.insert(k, v.into());
                }
                ContextMapValue::Map(new_val)
            }
            Document::Seq(s) => {
                let new_val: Vec<ContextMapValue> = s.into_iter().map(|i| i.into()).collect();
                ContextMapValue::Seq(new_val)
            }
            Document::Newtype(mut d) => d.take().into(),
            other => ContextMapValue::Node(Arc::new(Data::from(other).into())),
        }
    }
}

impl From<Node> for ContextMapValue {
    fn from(val: Node) -> Self {
        ContextMapValue::Node(Arc::new(val))
    }
}

impl From<Template> for ContextMapValue {
    fn from(val: Template) -> Self {
        ContextMapValue::Node(val.root_node())
    }
}

impl From<Data> for ContextMapValue {
    fn from(val: Data) -> Self {
        ContextMapValue::Node(Arc::new(val.into()))
    }
}
