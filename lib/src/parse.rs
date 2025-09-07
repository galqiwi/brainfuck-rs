use crate::instruction::Instruction;
use crate::instruction::Instruction::{
    Abort, BeginLoop, Decrement, EndLoop, GoLeft, GoRight, Increment, Input, Output,
};

pub fn parse(code: &str) -> Vec<Instruction> {
    let mut code = code.chars().enumerate();

    let mut output = Vec::new();
    let mut loop_stack: Vec<usize> = Vec::new();

    for (idx, c) in code {
        let new_instruction = match c {
            '>' => GoRight,
            '<' => GoLeft,
            '+' => Increment,
            '-' => Decrement,
            '.' => Output,
            ',' => Input,
            '[' => {
                loop_stack.push(idx);
                Abort
            }
            ']' => {
                let open_idx = loop_stack.pop().unwrap();
                output[open_idx] = BeginLoop(idx);
                EndLoop(open_idx)
            }
            _ => {
                continue;
            }
        };

        output.push(new_instruction);
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_instructions() {
        let result = parse("+-<>.,");
        assert_eq!(
            result,
            vec![Increment, Decrement, GoLeft, GoRight, Output, Input,]
        );
    }

    #[test]
    fn test_simple_loop() {
        let result = parse("[+]");
        assert_eq!(result, vec![BeginLoop(2), Increment, EndLoop(0),]);
    }

    #[test]
    fn test_nested_loops() {
        let result = parse("[[+]]");
        assert_eq!(
            result,
            vec![
                BeginLoop(4),
                BeginLoop(3),
                Increment,
                EndLoop(1),
                EndLoop(0),
            ]
        );
    }

    #[test]
    fn test_complex_program() {
        let result = parse("+[>+<-]");
        assert_eq!(
            result,
            vec![
                Increment,
                BeginLoop(6),
                GoRight,
                Increment,
                GoLeft,
                Decrement,
                EndLoop(1),
            ]
        );
    }

    #[test]
    fn test_ignore_non_brainfuck_chars() {
        let result = parse("+ hello world -");
        assert_eq!(result, vec![Increment, Decrement,]);
    }

    #[test]
    fn test_empty_string() {
        let result = parse("");
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_only_comments() {
        let result = parse("this is a comment");
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_multiple_loops() {
        let result = parse("[+][>]");
        assert_eq!(
            result,
            vec![
                BeginLoop(2),
                Increment,
                EndLoop(0),
                BeginLoop(5),
                GoRight,
                EndLoop(3),
            ]
        );
    }

    #[test]
    fn test_empty_loop() {
        let result = parse("[]");
        assert_eq!(result, vec![BeginLoop(1), EndLoop(0),]);
    }

    #[test]
    fn test_deeply_nested_loops() {
        let result = parse("[[[+]]]");
        assert_eq!(
            result,
            vec![
                BeginLoop(6),
                BeginLoop(5),
                BeginLoop(4),
                Increment,
                EndLoop(2),
                EndLoop(1),
                EndLoop(0),
            ]
        );
    }
}
