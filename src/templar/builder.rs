use super::*;

/// Use TemplarBuilder for creating a new customized instance of Templar.
///
/// # Usage
///
/// ```
/// # use templar::*;
/// let builder = TemplarBuilder::default(); // Default filters/functions preloaded
/// let templar = builder.build();
/// ```
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
    /// Create a new empty context with no filters or functions. Generally, you should be using the
    /// default context.
    pub fn new() -> TemplarBuilder {
        TemplarBuilder {
            functions: Default::default(),
            filters: Default::default(),
        }
    }

    /// Add a function to the configuration with the name specified
    pub fn add_function<T: 'static + Fn(TemplarResult) -> TemplarResult + Send + Sync>(
        &mut self,
        name: &str,
        val: T,
    ) -> &mut Self {
        self.functions.insert(name.into(), Arc::new(val));
        self
    }

    /// Add a function to the configuration using Serde to make the function call generic
    #[cfg(feature = "generics")]
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

    /// Remove the specified function name from the configuration
    pub fn remove_function(&mut self, name: &str) -> &mut Self {
        self.functions.remove(name);
        self
    }

    /// Add a filter to the configuration with the specified signature
    pub fn add_filter<T: 'static + Fn(Data, Data) -> Data + Send + Sync>(
        &mut self,
        name: &str,
        val: T,
    ) -> &mut Self {
        self.filters.insert(name.into(), Arc::new(val));
        self
    }

    /// Add a filter to the configuration using serde to call it generically
    // #[cfg(feature = "generics")]
    // pub fn add_generic_filter<
    //     'de,
    //     T: 'static + serde::Deserialize<'de>,
    //     U: 'static + serde::Deserialize<'de>,
    //     V: 'static + serde::Serialize,
    // >(
    //     &mut self,
    //     name: &str,
    //     inner: GenericFilter<T, U, V>,
    // ) -> &mut Self {
    //     let generic_filter = move |a: TemplarResult, b: TemplarResult| {
    //         let arg1: T = a?.try_into().map_err(|e| {
    //             TemplarError::RenderFailure(format!("Arguments could not be deserialized: {}", e))
    //         })?;
    //         let arg2: U = b?.try_into().map_err(|e| {
    //             TemplarError::RenderFailure(format!("Arguments could not be deserialized: {}", e))
    //         })?;
    //         let result = inner(arg1, arg2)?;
    //         Ok(Document::new(result).map_err(|e| {
    //             TemplarError::RenderFailure(format!(
    //                 "Could not serialize result into Document: {}",
    //                 e
    //             ))
    //         })?)
    //     };
    //     self.filters.insert(name.into(), Arc::new(generic_filter));
    //     self
    // }

    /// Remove the specified filter name from the configuration
    pub fn remove_filter(&mut self, name: &str) -> &mut Self {
        self.filters.remove(name);
        self
    }

    /// Build a new templar instance with this configuration
    pub fn build(self) -> Templar {
        let functions = self.functions;
        let filters = self.filters;
        Templar { functions, filters }
    }
}
