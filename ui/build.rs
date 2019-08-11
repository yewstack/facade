use failure::Error;
use sass_rs::{compile_file, Options, OutputStyle};
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Error> {
    let data = compile_file("./styles/styles.scss", Options::default()).unwrap();
    let mut file = File::create("./static/styles.css")?;
    file.write_all(data.as_bytes())?;
    Ok(())
}
