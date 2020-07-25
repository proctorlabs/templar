use super::*;
use command::*;
use context::build_context;
use std::io::prelude::*;
use std::path::PathBuf;
use std::fs::create_dir;
use templar::Templar;
use util::*;

mod command;
mod context;
mod util;

pub fn run() -> Result<()> {
    CommandContext::new(Command::parse()?)?.run()
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

    fn run(&self) -> Result<()> {
        match (&self.cmd.expr, &self.cmd.template) {
            (Some(ref text), None) => self.exec_expression(text),
            (None, Some(ref file)) => self.exec_path(file),
            (None, None) => self.exec_stdin(),
            _ => unreachable!(), //Command:parse() has these as mutually exclusive
        }
    }

    fn exec_path(&self, file: &PathBuf) -> Result<()> {
        if file.is_file() {
            let template_contents = read_file(file)?;
            self.render_file(Templar::global().parse_template(&template_contents)?)
        } else if file.is_dir() {
            if self.cmd.recursive {
                if self.cmd.destination.is_none()
                    || !self.cmd.destination.as_ref().unwrap().is_dir()
                {
                    Err(TemplarError::RenderFailure(
                        "When templating a directory, the destination must also be a directory"
                            .into(),
                    ))
                } else {
                    self.render_recursive(file, self.cmd.destination.as_ref().unwrap())
                }
            } else {
                Err(TemplarError::RenderFailure(
                    "Recursive flag must be used to template a directory.".into(),
                ))
            }
        } else {
            Err(TemplarError::RenderFailure("Template not found!".into()))
        }
    }

    fn exec_expression(&self, text: &str) -> Result<()> {
        self.render_file(Templar::global().parse_expression(text)?)
    }

    fn exec_stdin(&self) -> Result<()> {
        let template_contents = read_stdin()?;
        self.render_file(Templar::global().parse_template(&template_contents)?)
    }

    fn render_recursive(&self, src: &PathBuf, dst: &PathBuf) -> Result<()> {
        if src.is_dir() {
            if !dst.exists() {
                create_dir(dst)?;
            }
            for entry in src.read_dir()? {
                let p = entry?.path();
                let filename = p.file_name().unwrap();
                let mut newsrc = src.clone();
                let mut newdst = dst.clone();
                newsrc.push(filename);
                newdst.push(filename);
                self.render_recursive(&newsrc, &newdst)?;
            }
            Ok(())
        } else {
            let template_contents = read_file(src)?;
            let tpl = Templar::global().parse_template(&template_contents)?;
            let output = tpl.render(&self.ctx)?;
            write_file(dst, &output)
        }
    }

    fn render_file(&self, tpl: Template) -> Result<()> {
        let output = tpl.render(&self.ctx)?;
        match self.cmd.destination {
            Some(ref file) => write_file(file, &output),
            None => write_stdout(&output),
        }
    }
}
