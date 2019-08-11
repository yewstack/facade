use failure::{err_msg, Error};
use sass_rs::{compile_file, Options, OutputStyle};
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Error> {
    let mut opts = Options::default();
    opts.output_style = OutputStyle::Compact;
    let data = compile_file("./styles/styles.scss", opts).map_err(err_msg)?;
    let mut file = File::create("./static/styles.css")?;
    file.write_all(data.as_bytes())?;
    Ok(())
}
