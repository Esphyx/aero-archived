use front_end::{
    grammar::{ast::SourceAST, lexer::Lexer, parser::Parser},
    kernel::{checker::reducer, de_bruijn::program::Program},
};

pub mod back_end;
pub mod front_end;

// TODO
// constant folding
// three-address code
// register allocation

// LEXER -> PARSER -> SOURCE AST -> DE BRUIJN & GLOBAL NAME RES -> TYPE CHECKING

pub fn compile(input: String) -> Result<(), Box<dyn std::error::Error>> {
    let tokens = Lexer::new(input.clone()).tokens()?;

    // println!("{:#?}", tokens);

    let mut parser = Parser::new(tokens, input.clone())?;

    let ast = SourceAST::parse(&mut parser).map_err(|e| e.with_source(input))?;

    println!("{:#?}", ast);

    let program = Program::from(&ast);

    println!("{:#?}", program);

    let reduced = reducer(program);

    println!("{:#?}", reduced);

    Ok(())
}
