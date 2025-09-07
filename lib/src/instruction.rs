#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Instruction {
    GoRight,
    GoLeft,
    Increment,
    Decrement,
    Output,
    Input,
    BeginLoop(usize),
    EndLoop(usize),
    Abort,
}
