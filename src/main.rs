use crate::{
    back_end::{assembler::assemble, generator::generate, linker::link},
    front_end::{lexer::LexicalAnalizer, parser::Parser},
};

mod back_end;
mod front_end;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input.aero");

    let lexer = LexicalAnalizer::new(input);
    let mut parser = Parser::new(lexer)?;

    let ast = parser.parse()?;

    println!("{:#?}", ast);

    let instructions = generate(&ast);

    println!("{:?}", instructions);

    let binary = assemble(instructions);

    link(binary)?;

    Ok(())
}
