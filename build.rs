use failure::{format_err, Error};
use std::process::Command;

trait RunIt {
    fn run_it(&mut self, err: &str) -> Result<(), Error>;
}

impl RunIt for Command {
    fn run_it(&mut self, err: &str) -> Result<(), Error> {
        let output = self.output()?;
        if !output.status.success() {
            let out = String::from_utf8_lossy(&output.stderr);
            eprintln!("{}", out);
            Err(format_err!("{}", err))
        } else {
            Ok(())
        }
    }
}

fn run(mut command: Command, err: &str) -> Result<(), Error> {
    let output = command.output()?;
    if !output.status.success() {
        let out = String::from_utf8_lossy(&output.stderr);
        eprintln!("{}", out);
        Err(format_err!("{}", err))
    } else {
        Ok(())
    }
}

fn main() -> Result<(), Error> {
    Command::new("cargo")
        .args(&["web", "deploy"])
        .current_dir("ui")
        .run_it("Can't compile UI")?;

    Command::new("tar")
        .args(&["-cvf", "target/ui.tar", "-C", "target/deploy", "."])
        .current_dir("ui")
        .run_it("Can't pack UI")?;

    Ok(())
}
