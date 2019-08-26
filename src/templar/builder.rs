use super::*;

pub struct TemplarBuilder {
    functions: HashMap<String, Function>,
    filters: HashMap<String, Filter>,
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

    pub fn add_function(&mut self, name: &str, val: Function) -> &mut Self {
        self.functions.insert(name.into(), val);
        self
    }

    pub fn remove_function(&mut self, name: &str) -> &mut Self {
        self.functions.remove(name);
        self
    }

    pub fn add_filter(&mut self, name: &str, val: Filter) -> &mut Self {
        self.filters.insert(name.into(), val);
        self
    }

    pub fn remove_filter(&mut self, name: &str) -> &mut Self {
        self.filters.remove(name);
        self
    }

    pub fn build(self) -> Templar {
        let functions = self
            .functions
            .into_iter()
            .map(|(k, v)| (k, Arc::new(v)))
            .collect();

        let filters = self
            .filters
            .into_iter()
            .map(|(k, v)| (k, Arc::new(v)))
            .collect();

        Templar { functions, filters }
    }
}
