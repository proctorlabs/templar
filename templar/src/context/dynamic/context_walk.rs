use super::*;

pub struct ContextWalk<'a> {
    inner: std::cell::RefCell<ContextWalkValue<'a>>,
}

impl<'a> ContextWalk<'a> {
    pub fn exec(&self, ctx: &impl Context) -> Data {
        let val = &*self.inner.borrow();
        match val {
            ContextWalkValue::None => Data::empty(),
            ContextWalkValue::Owned(v) => v.exec(ctx),
            ContextWalkValue::Ref(v) => v.exec(ctx),
        }
    }

    pub fn walk(&'a self, ctx: &impl Context, key: &Document) {
        let val = self.inner.borrow();
        let mut new_walk = None;
        let mut res = None;
        match &*val {
            ContextWalkValue::Ref(ContextMapValue::Map(m)) => new_walk = Some(m.get(key).into()),
            ContextWalkValue::Ref(val) => res = Some(val.exec(ctx)),
            ContextWalkValue::Owned(val) => res = Some(val.exec(ctx)),
            _ => return,
        };
        drop(val);
        if let Some(res) = res {
            new_walk = Some(if res.is_failed() || res.is_empty() {
                res.into()
            } else {
                match res.into_result() {
                    Ok(Document::Map(m)) => m.get(key).cloned().into(),
                    Ok(other) => other.into(),
                    Err(e) => e.into(),
                }
            })
        }
        let inner_mut: &mut ContextWalkValue<'a> = &mut self.inner.borrow_mut();
        drop(replace(inner_mut, new_walk.unwrap()));
    }
}

impl<'a> From<ContextWalkValue<'a>> for ContextWalk<'a> {
    fn from(val: ContextWalkValue<'a>) -> Self {
        ContextWalk {
            inner: std::cell::RefCell::new(val),
        }
    }
}

impl<'a> From<Option<&'a ContextMapValue>> for ContextWalk<'a> {
    fn from(val: Option<&'a ContextMapValue>) -> Self {
        match val {
            Some(v) => ContextWalkValue::Ref(v).into(),
            None => ContextWalkValue::from(Data::empty()).into(),
        }
    }
}

enum ContextWalkValue<'a> {
    Ref(&'a ContextMapValue),
    Owned(ContextMapValue),
    None,
}

impl<'a> From<Option<&'a ContextMapValue>> for ContextWalkValue<'a> {
    fn from(val: Option<&'a ContextMapValue>) -> Self {
        match val {
            Some(v) => Self::Ref(v),
            None => Self::None,
        }
    }
}

impl<'a> From<Document> for ContextWalkValue<'a> {
    fn from(val: Document) -> Self {
        Self::Owned(Data::from(val).into())
    }
}

impl<'a> From<TemplarError> for ContextWalkValue<'a> {
    fn from(val: TemplarError) -> Self {
        Self::Owned(Data::from(val).into())
    }
}

impl<'a> From<Option<Document>> for ContextWalkValue<'a> {
    fn from(val: Option<Document>) -> Self {
        match val {
            Some(v) => v.into(),
            None => Self::None,
        }
    }
}

impl<'a> From<Data> for ContextWalkValue<'a> {
    fn from(val: Data) -> Self {
        Self::Owned(val.into())
    }
}
