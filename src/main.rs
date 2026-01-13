use crate::{lexer::LexicalAnalizer, parser::Parser};

mod ast;
mod checker;
mod evaluator;
mod lexer;
mod parser;
mod token;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input.aero");

    let lexer = LexicalAnalizer::new(input);
    let mut parser = Parser::new(lexer)?;

    let expression = parser.parse_expression()?;

    // let expected_type = type_check(&expression, &HashMap::new())?;

    println!("{:#?}", expression);

    Ok(())
}
