use std::{error::Error, fs};

use ast::ast::SourceAST;
use hir::lowering::program::Program;
use lexer::lexer::Lexer;
use parser::parser::Parser;
use semantic::kernel::reducer;

fn main() -> Result<(), Box<dyn Error>> {
    compile(fs::read_to_string("example/src/main.aero")?)?;

    Ok(())
}

pub fn compile(input: String) -> Result<(), Box<dyn std::error::Error>> {
    let tokens = Lexer::new(input.clone()).tokens()?;

    println!("{:#?}", tokens);

    let mut parser = Parser::new(tokens, input.clone())?;

    let ast = SourceAST::parse(&mut parser).map_err(|e| e.with_source(input))?;

    let program = Program::from(&ast);

    println!("{:#?}", program);

    let reduced = reducer(program);

    println!("{:#?}", reduced);

    Ok(())
}
