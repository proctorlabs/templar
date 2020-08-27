use super::*;
use serde::{Deserialize, Serialize};
use unstructured::Document;
use std::collections::HashMap;
use templar::StandardContext;
use templar::Context;

#[derive(Debug, Default, Clone)]
pub struct Yaatfile {
    vars: Document,
    options: Document,
    tasks: HashMap<String, Task>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Task {
    Script(String),
    Script2{
        #[serde(default)]
        depends_on: Vec<String>,
        script: String,
        #[serde(default = "shell")]
        shell: String,
    },
}

#[derive(Debug, Default, Clone)]
pub struct Ctx {
    tasks: HashMap<String, Task>,
    pub args: YaatArgs,
    context: StandardContext,
}

impl Yaatfile {
    pub fn init(mut doc: Document, args: YaatArgs) -> Result<Ctx> {
        let opts = doc[".yaat"].replace("".into());
        let tree: TemplateTree = Templar::global().parse(&opts["options"])?;
        let ctx = StandardContext::new();
        ctx.set(tree)?;
        ctx.merge(opts["vars"].clone())?;
        let tasks: HashMap<String ,Task> = doc.try_into().unwrap_or_default();

        Ok(Ctx{
            tasks,
            context: ctx,
            args
        })
    }
}

impl Ctx {
    pub fn get_task(&self, name: &str) -> Option<Task> {
        self.tasks.get(name).cloned()
    }

    pub fn get_context(&self) -> Result<&StandardContext> {
        Ok(&self.context)
    }

    pub fn get_args(&self) -> Result<&YaatArgs> {
        Ok(&self.args)
    }
}

fn shell() -> String {
    "bash".into()
}
