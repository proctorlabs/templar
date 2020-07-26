use super::*;
use command::*;
use context::build_context;
use std::fs::{create_dir_all, remove_file};
use std::io::prelude::*;
use std::path::PathBuf;
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
            match (
                self.cmd.recursive,
                self.cmd.destination.as_ref(),
                self.cmd.force,
            ) {
                (false, _, _) => Err(TemplarError::RenderFailure(
                    "Recursive flag must be used to template a directory.".into(),
                )),
                (true, None, _) => Err(TemplarError::RenderFailure(
                    "Destination path required when templating into a directory".into(),
                )),
                (true, Some(d), true) => self.render_recursive(file, d),
                (true, Some(d), false) => {
                    if !d.exists() || d.is_dir() {
                        self.render_recursive(file, d)
                    } else {
                        Err(TemplarError::RenderFailure(
                            "Destination must be new path or existing directory. Use --force to allow overwriting existing content.".into(),
                        ))
                    }
                }
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
            if dst.is_file() && self.cmd.force {
                remove_file(dst)?;
            } else if !dst.exists() {
                create_dir_all(dst)?;
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
            if dst.is_file() {
                if self.cmd.force {
                    remove_file(dst)?;
                } else {
                    return Err(TemplarError::RenderFailure(format!(
                        "Destination file '{}' exists!",
                        dst.file_name().unwrap_or_default().to_string_lossy()
                    )));
                }
            }
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
