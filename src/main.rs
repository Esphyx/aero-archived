use std::{env, fs::File, io::Write, process::Command};

use crate::{
    back_end::{
        assembler::{ToBytes, assemble},
        generator::generate,
    },
    front_end::{lexer::LexicalAnalizer, parser::Parser},
};

mod back_end;
mod front_end;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input.aero");

    let lexer = LexicalAnalizer::new(input);
    let mut parser = Parser::new(lexer)?;

    let ast = parser.parse()?;
    let instructions = generate(&ast);
    let common_object = assemble(instructions);

    dbg!(&common_object);

    let project_dir = env::current_dir()?;
    let obj_path = project_dir.join("main.obj");
    let exe_path = project_dir.join("main.exe");

    let mut file = File::create(&obj_path)?;
    file.write_all(&common_object.to_bytes())?;

    let status = Command::new("gcc")
        .args(&[
            obj_path.to_str().unwrap(),
            "-o",
            exe_path.to_str().unwrap(),
            "-nostdlib",
            "-lkernel32",
        ])
        .status()?;

    if !status.success() {
        eprintln!("GCC failed");
        std::process::exit(-1);
    }

    let status = Command::new(exe_path).status()?;
    if let Some(exit_code) = status.code() {
        println!("main.exe exited with code: {}", exit_code);
    } else {
        eprintln!("Exit code failed");
        std::process::exit(-1);
    }

    Ok(())
}
