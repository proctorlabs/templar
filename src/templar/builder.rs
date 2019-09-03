use super::*;

pub struct TemplarBuilder {
    functions: HashMap<String, Arc<Function>>,
    filters: HashMap<String, Arc<Filter>>,
}

impl Default for TemplarBuilder {
    fn default() -> TemplarBuilder {
        TemplarBuilder {
            functions: extensions::default_functions(),
            filters: extensions::default_filters(),
        }
    }
}

impl TemplarBuilder {
    pub fn new() -> TemplarBuilder {
        TemplarBuilder {
            functions: Default::default(),
            filters: Default::default(),
        }
    }

    pub fn add_function<T: 'static + Fn(TemplarResult) -> TemplarResult + Send + Sync>(
        &mut self,
        name: &str,
        val: T,
    ) -> &mut Self {
        self.functions.insert(name.into(), Arc::new(val));
        self
    }

    pub fn add_generic_function<
        'de,
        T: 'static + serde::Deserialize<'de>,
        U: 'static + serde::Serialize,
    >(
        &mut self,
        name: &str,
        inner: GenericFunction<T, U>,
    ) -> &mut Self {
        let generic_fn = move |a: TemplarResult| {
            let sub_args: T = a?.try_into().map_err(|e| {
                TemplarError::RenderFailure(format!("Arguments could not be deserialized: {}", e))
            })?;
            let result = inner(sub_args)?;
            Ok(Document::new(result).map_err(|e| {
                TemplarError::RenderFailure(format!(
                    "Could not serialize result into Document: {}",
                    e
                ))
            })?)
        };
        self.functions.insert(name.into(), Arc::new(generic_fn));
        self
    }

    pub fn remove_function(&mut self, name: &str) -> &mut Self {
        self.functions.remove(name);
        self
    }

    pub fn add_filter<
        T: 'static + Fn(TemplarResult, TemplarResult) -> TemplarResult + Send + Sync,
    >(
        &mut self,
        name: &str,
        val: T,
    ) -> &mut Self {
        self.filters.insert(name.into(), Arc::new(val));
        self
    }

    pub fn add_generic_filter<
        'de,
        T: 'static + serde::Deserialize<'de>,
        U: 'static + serde::Deserialize<'de>,
        V: 'static + serde::Serialize,
    >(
        &mut self,
        name: &str,
        inner: GenericFilter<T, U, V>,
    ) -> &mut Self {
        let generic_filter = move |a: TemplarResult, b: TemplarResult| {
            let arg1: T = a?.try_into().map_err(|e| {
                TemplarError::RenderFailure(format!("Arguments could not be deserialized: {}", e))
            })?;
            let arg2: U = b?.try_into().map_err(|e| {
                TemplarError::RenderFailure(format!("Arguments could not be deserialized: {}", e))
            })?;
            let result = inner(arg1, arg2)?;
            Ok(Document::new(result).map_err(|e| {
                TemplarError::RenderFailure(format!(
                    "Could not serialize result into Document: {}",
                    e
                ))
            })?)
        };
        self.filters.insert(name.into(), Arc::new(generic_filter));
        self
    }

    pub fn remove_filter(&mut self, name: &str) -> &mut Self {
        self.filters.remove(name);
        self
    }

    pub fn build(self) -> Templar {
        let functions = self.functions;
        let filters = self.filters;
        Templar { functions, filters }
    }
}
