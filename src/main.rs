use std::{error::Error, fs};

use hir::lowering::namespace::Namespace;
use lexer::lexer::Lexer;
use parser::parser::Parser;
use semantic::kernel::Kernel;
use syntax::ast::Syntax;

fn main() -> Result<(), Box<dyn Error>> {
    compile(fs::read_to_string("example/src/main.aero")?);

    Ok(())
}

pub fn compile(input: String) {
    let tokens = Lexer::new(input.clone()).tokens().unwrap();
    let mut parser = Parser::new(tokens, input.clone()).unwrap();
    let syntax = Syntax::parse(&mut parser);

    let (namespace, context) = Namespace::from(&syntax);

    let kernel = Kernel::from(&namespace);
}
