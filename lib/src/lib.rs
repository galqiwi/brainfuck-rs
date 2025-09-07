use crate::interpreter::run_bytecode;
use std::io::Stdin;

pub mod instruction;
pub mod interpreter;
pub mod parse;

pub fn run(code: &str) {
    let bytecode = parse::parse(code);
    run_bytecode(&bytecode, std::io::stdin(), std::io::stdout());
}
