use std::{error::Error, fs};

mod compiler;

fn main() -> Result<(), Box<dyn Error>> {
    compiler::compile(fs::read_to_string("example/src/main.aero")?)?;

    Ok(())
}
