use std::{error::Error, fs};

use ast::ast::AST;
use hir::lowering::program::Program;
use lexer::lexer::Lexer;
use parser::parser::Parser;
use semantic::{kernel::whnf, positivity::check_program};

fn main() -> Result<(), Box<dyn Error>> {
    compile(fs::read_to_string("example/src/main.aero")?)?;

    Ok(())
}

pub fn compile(input: String) -> Result<(), Box<dyn std::error::Error>> {
    let tokens = Lexer::new(input.clone()).tokens()?;
    let mut parser = Parser::new(tokens, input.clone())?;
    let ast = AST::parse(&mut parser).map_err(|e| e.with_source(input))?;

    let program = Program::from(&ast);

    dbg!(&program.namespace.inductives);

    check_program(&program);

    dbg!(&program);

    let entry = program.entry_point_definition();

    let reduced = whnf(entry, &program);
    println!("{:#?}", reduced);

    Ok(())
}
