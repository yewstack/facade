use failure::{format_err, Error};
use std::env;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;
use std::process::{Command, Stdio};

trait RunIt {
    fn run_it(&mut self, err: &str) -> Result<(), Error>;
}

impl RunIt for Command {
    fn run_it(&mut self, err: &str) -> Result<(), Error> {
        self.stderr(Stdio::piped());
        let mut child = self.spawn()?;
        if let Some(out) = child.stdout.take() {
            let buf = BufReader::new(out);
            for line in buf.lines() {
                eprint!("{}", line?);
            }
        }
        if !child.wait()?.success() {
            if let Some(mut err) = child.stderr.take() {
                let mut out = String::new();
                err.read_to_string(&mut out)?;
                eprintln!("{}", out);
            }
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
    let tar_path = tar_path
        .to_str()
        .ok_or_else(|| format_err!("can't create path to archive"))?;

    Command::new("tar")
        .args(&["-cvzf", tar_path, "-C", "target/deploy", "."])
        .current_dir("ui")
        .run_it("Can't pack UI")?;

    if cfg!(feature = "refresh") {
        Command::new("touch")
            .args(&["build.rs"])
            .run_it("Can't touch the build file")?;
    }

    Ok(())
}
