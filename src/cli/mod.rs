use super::*;
use command::*;
use context::build_context;
use std::io::prelude::*;
use std::path::PathBuf;
use templar::Templar;
use util::*;

mod command;
mod context;
mod util;

pub fn run() -> Result<()> {
    let cmdctx = CommandContext::new(Command::parse()?)?;
    match (&cmdctx.cmd.expr, &cmdctx.cmd.template) {
        (Some(ref text), None) => cmdctx.exec_expression(text),
        (None, Some(ref file)) => cmdctx.exec_path(file),
        (None, None) => cmdctx.exec_stdin(),
        _ => unreachable!(), //Command:parse() has these as mutually exclusive
    }
}

#[derive(Debug)]
struct CommandContext {
    cmd: Command,
    ctx: StandardContext,
}

impl CommandContext {
    fn new(cmd: Command) -> Result<Self> {
        let ctx = build_context(&cmd)?;
        Ok(CommandContext { cmd, ctx })
    }

    fn exec_expression(&self, text: &str) -> Result<()> {
        self.render(Templar::global().parse_expression(text)?)
    }

    fn exec_path(&self, file: &PathBuf) -> Result<()> {
        let template_contents = read_file(file)?;
        self.render(Templar::global().parse_template(&template_contents)?)
    }

    fn exec_stdin(&self) -> Result<()> {
        let template_contents = read_stdin()?;
        self.render(Templar::global().parse_template(&template_contents)?)
    }

    fn render(&self, tpl: Template) -> Result<()> {
        let output = tpl.render(&self.ctx)?;
        match self.cmd.destination {
            Some(ref file) => write_file(file, &output),
            None => write_stdout(&output),
        }
    }
}
