use std::{env, fs::File, io::Write, process::Command};

use crate::{
    back_end::assembler::{ToBytes, coff::CommonObject},
    front_end::{
        grammar::{lexer::LexicalAnalyzer, parser::Parser},
        kernel::debruijn::convert_parser_ast_to_db,
    },
};

mod back_end;
mod front_end;

// constant folding
// three-address code
// register allocation

// LEXER -> PARSER -> SOURCE AST -> (DE BRUIJN INDICES & GLOBAL NAME RESOLUTION) -> TYPE CHECKING

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input.aero");

    let tokens = LexicalAnalyzer::new(input).tokens()?;

    let ast = Parser::new(tokens)?.parse()?;

    println!("{:#?}", ast);

    let ast_db = convert_parser_ast_to_db(&ast, input);

    // match ast_db {
    // front_end::kernel::debruijn_ast::ErrorDB::Unbound { id } => {
    // println!("{}", id.resolve(input))
    // },
    // }

    println!("{:#?}", ast_db);

    // let instructions = generate(&ast);
    // let common_object = assemble(instructions);

    // link_and_run(common_object)?;

    Ok(())
}

fn link_and_run(common_object: CommonObject) -> Result<(), Box<dyn std::error::Error>> {
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
