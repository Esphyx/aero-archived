use std::{error::Error, fs};

use hir::lowering::namespace::Namespace;
use lexer::lexer::Lexer;
use parser::parser::Parser;
use semantic::kernel::Kernel;
use syntax::ast::Syntax;

fn main() -> Result<(), Box<dyn Error>> {
    compile(fs::read_to_string("example/src/main.aero")?)?;

    Ok(())
}

pub fn compile(input: String) -> Result<(), Box<dyn std::error::Error>> {
    let tokens = Lexer::new(input.clone()).tokens()?;
    let mut parser = Parser::new(tokens, input.clone()).unwrap();
    let syntax = Syntax::parse(&mut parser).unwrap();

    let (namespace, context) = Namespace::from(&syntax);

    let kernel = Kernel::new(&namespace);

    Ok(())
}
