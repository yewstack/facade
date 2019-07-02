use failure::{format_err, Error};
use std::env;
use std::path::PathBuf;
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

fn main() -> Result<(), Error> {
    Command::new("cargo")
        .args(&["web", "deploy"])
        .current_dir("ui")
        .run_it("Can't compile UI")?;

    let out_path = PathBuf::from(env::var("OUT_DIR")?);
    let tar_path = out_path.join("ui.tar.gz");
    let tar_path = tar_path.to_str()
        .ok_or_else(|| format_err!("can't create path to archive"))?;
    Command::new("tar")
        .args(&["-cvzf", tar_path, "-C", "target/deploy", "."])
        .current_dir("ui")
        .run_it("Can't pack UI")?;

    Ok(())
}
