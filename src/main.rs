use common::Error;
use parser::parse;
use stack::VecStack;
use tokenizer::tokenize;

mod common;
mod parser;
mod stack;
mod stack_machine;
mod tokenizer;

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    let input = &args[1];
    let input = std::fs::read_to_string(input).expect("Failed to read file");
    let tokens = tokenize(&input)?;
    let program = parse(tokens)?;
    let result = program.execute(&mut VecStack::new());
    println!("Result: {:?}", result);
    Ok(())
}
