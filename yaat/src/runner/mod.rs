use super::*;
use crate::util::read_file;
use std::env;
use std::process::{Command, Stdio};

pub fn run(args: YaatArgs) -> Result<()> {
    let mut yaatfile = env::current_dir()?;
    yaatfile.push(".yaatfile");
    while !yaatfile.is_file() {
        yaatfile.pop(); // Remove the .yaatfile that we just added
        if !yaatfile.pop() {
            // move up one directory
            return Err(TemplarError::IO("No .yaatfile found!".into()));
        }
        env::set_current_dir(&yaatfile)?; // Change our working directory to the current directory we are searching
        yaatfile.push(".yaatfile"); // Add the .yaatfile to the path to check in the next loop
    }
    let contents = read_file(&yaatfile)?;
    let yaatfile: Document = serde_yaml::from_str(&contents).unwrap_or_default();
    run_file(Yaatfile::init(yaatfile, args)?)
}

fn run_file(ctx: Ctx) -> Result<()> {
    let taskname = &ctx.get_args()?.taskname;
    let task = ctx
        .get_task(taskname)
        .ok_or_else(|| TemplarError::IO(taskname.to_string()))?;
    match task {
        Task::Script(scr) => {
            let jit = Templar::global().parse(&scr)?;
            let result = jit.render(ctx.get_context()?)?;
            run_script("sh", &result)
        }
        Task::Script2 {
            depends_on,
            shell,
            script,
        } => {
            for dep in depends_on.iter() {
                let mut new_ctx = ctx.clone();
                new_ctx.args.taskname = dep.to_string();
                run_file(new_ctx)?;
            }
            let jit = Templar::global().parse(&script)?;
            let result = jit.render(ctx.get_context()?)?;
            run_script(&shell, &result)
        }
    }
}

fn run_script(shell: &str, script: &str) -> Result<()> {
    let code = Command::new("/usr/bin/env")
        .args(&[shell, "-c", script])
        .stdin(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()?
        .wait()?
        .code()
        .unwrap_or(2);
    if code != 0 {
        Err(TemplarError::IO(format!(
            "Script finished with code {}",
            code
        )))
    } else {
        Ok(())
    }
}
