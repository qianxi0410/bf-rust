use std::{
    env,
    fs::File,
    io::{self, Read},
};

#[derive(Debug, Clone)]
enum OpCode {
    IncrementPointer,
    DecrementPointer,
    Increment,
    Decrement,
    Write,
    Read,
    LoopBegin,
    LoopEnd,
}

#[derive(Debug, Clone)]
enum Instruction {
    IncrementPointer,
    DecrementPointer,
    Increment,
    Decrement,
    Write,
    Read,
    Loop(Vec<Instruction>),
}

// code lexer
// turn your code to op
fn lexer(source: String) -> Vec<OpCode> {
    let mut operations = vec![];

    for symbol in source.chars() {
        let op = match symbol {
            '>' => Some(OpCode::IncrementPointer),
            '<' => Some(OpCode::DecrementPointer),
            '+' => Some(OpCode::Increment),
            '-' => Some(OpCode::Decrement),
            '.' => Some(OpCode::Write),
            ',' => Some(OpCode::Read),
            '[' => Some(OpCode::LoopBegin),
            ']' => Some(OpCode::LoopEnd),
            _ => None,
        };

        match op {
            Some(op) => operations.push(op),
            None => (),
        }
    }

    operations
}

// turn your op vec to instruct vec
fn parser(opcodes: Vec<OpCode>) -> Vec<Instruction> {
    let mut program = Vec::new();
    let mut loop_stack = 0;
    let mut loop_start = 0;

    for (i, op) in opcodes.iter().enumerate() {
        if loop_stack == 0 {
            let instr = match op {
                OpCode::IncrementPointer => Some(Instruction::IncrementPointer),
                OpCode::DecrementPointer => Some(Instruction::DecrementPointer),
                OpCode::Increment => Some(Instruction::Increment),
                OpCode::Decrement => Some(Instruction::Decrement),
                OpCode::Write => Some(Instruction::Write),
                OpCode::Read => Some(Instruction::Read),

                OpCode::LoopBegin => {
                    loop_start = i;
                    loop_stack += 1;
                    None
                }

                OpCode::LoopEnd => panic!("loop ending at #{} has no beginning", i),
            };

            match instr {
                Some(instr) => program.push(instr),
                None => (),
            }
        } else {
            match op {
                OpCode::LoopBegin => {
                    loop_stack += 1;
                }
                OpCode::LoopEnd => {
                    loop_stack -= 1;

                    if loop_stack == 0 {
                        program.push(Instruction::Loop(parser(
                            opcodes[loop_start + 1..i].to_vec(),
                        )));
                    }
                }
                _ => (),
            }
        }
    }

    if loop_stack != 0 {
        panic!(
            "loop that starts at #{} has no matching ending!",
            loop_start
        );
    }

    program
}

// Executes a program that was previously parsed
fn eval(instructions: &Vec<Instruction>, paper: &mut Vec<u8>, data_pointer: &mut usize) {
    for instr in instructions {
        match instr {
            Instruction::IncrementPointer => *data_pointer += 1,
            Instruction::DecrementPointer => *data_pointer -= 1,
            Instruction::Increment => paper[*data_pointer] += 1,
            Instruction::Decrement => paper[*data_pointer] -= 1,
            Instruction::Write => print!("{}", paper[*data_pointer] as char),
            Instruction::Read => {
                let mut input: [u8; 1] = [0; 1];
                std::io::stdin()
                    .read_exact(&mut input)
                    .expect("failed to read stdin");
                paper[*data_pointer] = input[0];
            }
            Instruction::Loop(nested_instructions) => {
                while paper[*data_pointer] != 0 {
                    eval(&nested_instructions, paper, data_pointer)
                }
            }
        }
    }
}

pub fn fuck(code: String) {
    let opcodes = lexer(code);

    let program = parser(opcodes);

    let mut paper: Vec<u8> = vec![0; 1204 * 10];
    let mut data_pointer = 1024 * 5;

    eval(&program, &mut &mut paper, &mut data_pointer);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hello_world() {
        fuck("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.
".to_string());
    }
}
