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
            functions: functions::default_functions(),
            filters: filters::default_filters(),
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
    pub fn add_function<T: 'static + Fn(Data) -> Data + Send + Sync>(
        &mut self,
        name: &str,
        val: T,
    ) -> &mut Self {
        self.functions.insert(name.into(), Arc::new(val));
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
