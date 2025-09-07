use crate::instruction::Instruction;
use std::io::{Read, Write};

#[derive(Debug)]
struct State {
    memory: Vec<u8>,
    position: usize,
}

impl State {
    pub fn new() -> Self {
        State {
            memory: vec![0u8; 1024],
            position: 0,
        }
    }

    pub fn get_data(&self) -> u8 {
        self.memory[self.position]
    }
    pub fn set_data(&mut self, data: u8) {
        self.memory[self.position] = data;
    }

    pub fn move_left(&mut self) {
        assert_ne!(self.position, 0);
        self.position -= 1;
    }

    pub fn move_right(&mut self) {
        self.position += 1;
        if self.position == self.memory.len() {
            self.memory.push(0);
        }
    }
}

pub fn run_bytecode(bytecode: &[Instruction], mut input: impl Read, mut output: impl Write) {
    let mut state = State::new();

    let mut position: usize = 0;

    while position < bytecode.len() {
        let instruction = bytecode[position];
        match instruction {
            Instruction::GoRight => state.move_right(),
            Instruction::GoLeft => state.move_left(),
            Instruction::Increment => state.set_data(state.get_data().wrapping_add(1)),
            Instruction::Decrement => state.set_data(state.get_data().wrapping_sub(1)),
            Instruction::Output => {
                let buf = [state.get_data()];
                output.write_all(&buf).unwrap();
            }
            Instruction::Input => {
                let mut buf = [0];
                input.read_exact(&mut buf).unwrap();
                state.set_data(buf[0]);
            }
            Instruction::BeginLoop(idx) => {
                if state.get_data() == 0 {
                    position = idx;
                }
            }
            Instruction::EndLoop(idx) => {
                if state.get_data() != 0 {
                    position = idx;
                }
            }
            Instruction::Abort => {
                panic!();
            }
        }
        position += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Instruction::*;
    use std::io::Cursor;

    #[test]
    fn test_state_new() {
        let state = State::new();
        assert_eq!(state.position, 0);
        assert_eq!(state.get_data(), 0);
        assert_eq!(state.memory.len(), 1024);
    }

    #[test]
    fn test_state_increment_decrement() {
        let mut state = State::new();

        assert_eq!(state.get_data(), 0);
        state.set_data(state.get_data().wrapping_add(1));
        assert_eq!(state.get_data(), 1);
        state.set_data(state.get_data().wrapping_sub(1));
        assert_eq!(state.get_data(), 0);
    }

    #[test]
    fn test_state_wrapping() {
        let mut state = State::new();

        state.set_data(255);
        state.set_data(state.get_data().wrapping_add(1));
        assert_eq!(state.get_data(), 0);

        state.set_data(0);
        state.set_data(state.get_data().wrapping_sub(1));
        assert_eq!(state.get_data(), 255);
    }

    #[test]
    fn test_state_move_right() {
        let mut state = State::new();
        assert_eq!(state.position, 0);

        state.move_right();
        assert_eq!(state.position, 1);

        state.move_right();
        assert_eq!(state.position, 2);
    }

    #[test]
    fn test_state_move_left() {
        let mut state = State::new();
        state.move_right();
        state.move_right();
        assert_eq!(state.position, 2);

        state.move_left();
        assert_eq!(state.position, 1);

        state.move_left();
        assert_eq!(state.position, 0);
    }

    #[test]
    #[should_panic]
    fn test_state_move_left_panic() {
        let mut state = State::new();
        state.move_left();
    }

    #[test]
    fn test_state_memory_expansion() {
        let mut state = State::new();
        let initial_len = state.memory.len();

        for _ in 0..initial_len {
            state.move_right();
        }

        assert_eq!(state.memory.len(), initial_len + 1);
        assert_eq!(state.get_data(), 0);
    }

    #[test]
    fn test_basic_increment() {
        let bytecode = vec![Increment, Increment, Increment];
        let input = Cursor::new(Vec::new());
        let mut output = Vec::new();

        run_bytecode(&bytecode, input, &mut output);
        assert_eq!(output.len(), 0);
    }

    #[test]
    fn test_output() {
        let bytecode = vec![
            Increment, Increment, Increment, Increment, Increment, Increment, Increment, Increment,
            Increment, Increment, Increment, Increment, Increment, Increment, Increment, Increment,
            Increment, Increment, Increment, Increment, Increment, Increment, Increment, Increment,
            Increment, Increment, Increment, Increment, Increment, Increment, Increment, Increment,
            Increment, Increment, Increment, Increment, Increment, Increment, Increment, Increment,
            Increment, Increment, Increment, Increment, Increment, Increment, Increment, Increment,
            Increment, Increment, Increment, Increment, Increment, Increment, Increment, Increment,
            Increment, Increment, Increment, Increment, Increment, Increment, Increment, Increment,
            Increment, Output,
        ];
        let input = Cursor::new(Vec::new());
        let mut output = Vec::new();

        run_bytecode(&bytecode, input, &mut output);
        assert_eq!(output, vec![65]); // ASCII 'A'
    }

    #[test]
    fn test_input() {
        let bytecode = vec![Input, Output];
        let input = Cursor::new(vec![72]); // ASCII 'H'
        let mut output = Vec::new();

        run_bytecode(&bytecode, input, &mut output);
        assert_eq!(output, vec![72]);
    }

    #[test]
    fn test_simple_loop_skip() {
        let bytecode = vec![BeginLoop(2), Increment, EndLoop(0)];
        let input = Cursor::new(Vec::new());
        let mut output = Vec::new();

        run_bytecode(&bytecode, input, &mut output);
        assert_eq!(output.len(), 0);
    }

    #[test]
    fn test_simple_loop_execute() {
        let bytecode = vec![
            Increment,
            Increment,
            Increment,
            BeginLoop(6),
            Decrement,
            Output,
            EndLoop(3),
        ];
        let input = Cursor::new(Vec::new());
        let mut output = Vec::new();

        run_bytecode(&bytecode, input, &mut output);
        assert_eq!(output, vec![2, 1, 0]);
    }

    #[test]
    fn test_move_and_set() {
        let bytecode = vec![
            Increment, Increment, Increment, GoRight, Increment, Increment, Output, GoLeft, Output,
        ];
        let input = Cursor::new(Vec::new());
        let mut output = Vec::new();

        run_bytecode(&bytecode, input, &mut output);
        assert_eq!(output, vec![2, 3]);
    }

    #[test]
    fn test_hello_world_pattern() {
        let bytecode = vec![
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            BeginLoop(21),
            GoRight,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            GoRight,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            Increment,
            GoLeft,
            GoLeft,
            Decrement,
            EndLoop(10),
            GoRight,
            GoRight,
            Output,
        ];
        let input = Cursor::new(Vec::new());
        let mut output = Vec::new();

        run_bytecode(&bytecode, input, &mut output);
        assert_eq!(output[0], 100); // Should be close to 'd' or similar
    }

    #[test]
    fn test_empty_program() {
        let bytecode = vec![];
        let input = Cursor::new(Vec::new());
        let mut output = Vec::new();

        run_bytecode(&bytecode, input, &mut output);
        assert_eq!(output.len(), 0);
    }

    #[test]
    #[should_panic]
    fn test_abort_instruction() {
        let bytecode = vec![Abort];
        let input = Cursor::new(Vec::new());
        let mut output = Vec::new();

        run_bytecode(&bytecode, input, &mut output);
    }
}
