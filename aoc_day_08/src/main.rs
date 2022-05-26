use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
enum Instruction {
    NOP(isize),
    ACC(isize),
    JMP(isize),
}

struct CPU {
    pc: usize,
    accumulator: isize,
}

impl CPU {
    fn run(&mut self, instructions: &[Instruction]) -> bool {
        let mut executed = HashSet::new();

        loop {
            if !executed.insert(self.pc) {
                return false;
            }
            if self.pc >= instructions.len() {
                return true;
            }

            match instructions[self.pc] {
                Instruction::NOP(_) => self.pc += 1,
                Instruction::ACC(arg) => {
                    self.accumulator += arg;
                    self.pc += 1;
                }
                Instruction::JMP(arg) => self.pc = (self.pc as isize + arg) as usize,
            }
        }
    }
}

fn decode_input(input: &str) -> Vec<Instruction> {
    input.lines().map(|l| decode_line(l)).collect()
}

fn decode_line(line: &str) -> Instruction {
    let mut input = line.split(' ');
    match (
        input.next(),
        input.next().map(|arg| arg.parse::<isize>().unwrap()),
    ) {
        (Some("nop"), Some(arg)) => Instruction::NOP(arg),
        (Some("acc"), Some(arg)) => Instruction::ACC(arg),
        (Some("jmp"), Some(arg)) => Instruction::JMP(arg),
        _ => unimplemented!(),
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let program = decode_input(input);

    let mut cpu = CPU {
        pc: 0,
        accumulator: 0,
    };
    cpu.run(&program);

    println!("Part 1: {}", cpu.accumulator);

    let result = (0..program.len())
        .filter_map(|i| fix_program(&program, i))
        .find_map(|p| {
            let mut cpu = CPU {
                pc: 0,
                accumulator: 0,
            };
            if cpu.run(&p) {
                Some(cpu.accumulator)
            } else {
                None
            }
        });

    println!("Part2: acc: {}", result.unwrap());
}

fn fix_program(program: &[Instruction], i: usize) -> Option<Vec<Instruction>> {
    match program[i] {
        Instruction::JMP(arg) => {
            let mut program: Vec<Instruction> = program.to_vec();
            program[i] = Instruction::NOP(arg);
            Some(program)
        }
        Instruction::NOP(arg) => {
            let mut program = program.to_vec();
            program[i] = Instruction::JMP(arg);
            Some(program)
        }
        _ => None,
    }
}
