use std::{error::Error, fs};

use ast::ast::AST;
use hir::lowering::namespace::Namespace;
use lexer::lexer::Lexer;
use parser::parser::Parser;
use semantic::{context::Context, positivity::infer_type};

fn main() -> Result<(), Box<dyn Error>> {
    compile(fs::read_to_string("example/src/main.aero")?)?;

    Ok(())
}

pub fn compile(input: String) -> Result<(), Box<dyn std::error::Error>> {
    let tokens = Lexer::new(input.clone()).tokens()?;
    let mut parser = Parser::new(tokens, input.clone()).unwrap();
    let ast = AST::parse(&mut parser).unwrap();

    let namespace = Namespace::from(&ast);

    let expr = &namespace.functions[0].definition;

    let typ = &infer_type(expr, &mut Context::new(), &namespace);

    println!("{:?}", expr);
    println!("{:?}", typ);

    Ok(())
}
