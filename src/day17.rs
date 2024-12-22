#![allow(clippy::comparison_chain)]
use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone)]
pub struct Machine {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    reg_cp: usize,
    instruction_memory: Vec<u8>,
    output_buffer: Vec<u8>,
}

enum ComboOperand {
    Literal(i64),
    RegA,
    RegB,
    RegC,
    Invalid,
}

impl ComboOperand {
    fn parse(input: u8) -> Self {
        match input {
            0 => ComboOperand::Literal(0),
            1 => ComboOperand::Literal(1),
            2 => ComboOperand::Literal(2),
            3 => ComboOperand::Literal(3),
            4 => ComboOperand::RegA,
            5 => ComboOperand::RegB,
            6 => ComboOperand::RegC,
            _ => ComboOperand::Invalid,
        }
    }
}

enum Instruction {
    Adv(ComboOperand),
    Bxl(i64),
    Bst(ComboOperand),
    Jnz(i64),
    Bxc,
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand),
    Invalid,
}

impl Machine {
    fn next_instruction(&self) -> Option<Instruction> {
        let (opcode, operand) = if self.reg_cp < self.instruction_memory.len() - 1 {
            (self.instruction_memory[self.reg_cp], self.instruction_memory[self.reg_cp + 1])
        } else {
            return None;
        };


        Some(match opcode {
            0 => Instruction::Adv(ComboOperand::parse(operand)),
            1 => Instruction::Bxl(operand as i64),
            2 => Instruction::Bst(ComboOperand::parse(operand)),
            3 => Instruction::Jnz(operand as i64),
            4 => Instruction::Bxc,
            5 => Instruction::Out(ComboOperand::parse(operand)),
            6 => Instruction::Bdv(ComboOperand::parse(operand)),
            7 => Instruction::Cdv(ComboOperand::parse(operand)),
            _ => Instruction::Invalid,
        })
    }

    fn do_instruction(&mut self, ins: Instruction) -> bool {
        match ins {
            Instruction::Adv(combo_operand) => {
                let numerator = self.reg_a;
                let exponent = match combo_operand {
                    ComboOperand::Literal(v) => v,
                    ComboOperand::RegA => self.reg_a,
                    ComboOperand::RegB => self.reg_b,
                    ComboOperand::RegC => self.reg_c,
                    ComboOperand::Invalid => {
                        return false;
                    }
                };
                self.reg_a = numerator >> exponent;
                self.reg_cp += 2;
            },
            Instruction::Bxl(v) => {
                self.reg_b ^= v;
                self.reg_cp += 2;
            }
            Instruction::Bst(combo_operand) => {
                let op = match combo_operand {
                    ComboOperand::Literal(v) => v,
                    ComboOperand::RegA => self.reg_a,
                    ComboOperand::RegB => self.reg_b,
                    ComboOperand::RegC => self.reg_c,
                    ComboOperand::Invalid => {
                        return false;
                    }
                };
                self.reg_b = op & 0b111;
                self.reg_cp += 2;
            },
            Instruction::Jnz(v) => {
                if self.reg_a != 0 {
                    self.reg_cp = v as usize;
                } else {
                    self.reg_cp += 2;
                }
            },
            Instruction::Bxc => {
                self.reg_b ^= self.reg_c;
                self.reg_cp += 2;
            }
            Instruction::Out(combo_operand) => {
                let op = match combo_operand {
                    ComboOperand::Literal(v) => v,
                    ComboOperand::RegA => self.reg_a,
                    ComboOperand::RegB => self.reg_b,
                    ComboOperand::RegC => self.reg_c,
                    ComboOperand::Invalid => {
                        return false;
                    }
                };
                self.output_buffer.push((op & 0b111) as u8);
                self.reg_cp += 2;
            }
            Instruction::Bdv(combo_operand) => {
                let numerator = self.reg_a;
                let exponent = match combo_operand {
                    ComboOperand::Literal(v) => v,
                    ComboOperand::RegA => self.reg_a,
                    ComboOperand::RegB => self.reg_b,
                    ComboOperand::RegC => self.reg_c,
                    ComboOperand::Invalid => {
                        return false;
                    }
                };
                self.reg_b = numerator >> exponent;
                self.reg_cp += 2;
            }
            Instruction::Cdv(combo_operand) => {
                let numerator = self.reg_a;
                let exponent = match combo_operand {
                    ComboOperand::Literal(v) => v,
                    ComboOperand::RegA => self.reg_a,
                    ComboOperand::RegB => self.reg_b,
                    ComboOperand::RegC => self.reg_c,
                    ComboOperand::Invalid => {
                        return false;
                    }
                };
                self.reg_c = numerator >> exponent;
                self.reg_cp += 2;
            }
            Instruction::Invalid => return false,
        }
        true
    }
}

#[aoc_generator(day17)]
pub fn generator(input: &str) -> Machine {
    let (init_state_str, program_str) = input.split_once("\n\n").unwrap();
    let registers: Vec<i64> = init_state_str.lines().map(|l| {
        let parts = l.split_once(':').unwrap();
        parts.1.trim().parse().unwrap()
    }).collect();

    let parts = program_str.split_once(':').unwrap();
    let program_memmory = parts.1.trim().split(',').map(|n| {
        println!("n: {n}");
        n.parse().unwrap()
    }).collect();
    Machine {
        reg_a: registers[0],
        reg_b: registers[1],
        reg_c: registers[2],
        reg_cp: 0,
        instruction_memory: program_memmory,
        output_buffer: vec![]
    }
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &Machine) -> String {
    let mut machine = input.clone();
    while let Some(ins) = machine.next_instruction() {
        if !machine.do_instruction(ins) {
            break;
        }
    }
    let mut output = machine.output_buffer[0].to_string();
    for n in &machine.output_buffer[1..] {
        output += ",";
        output += &n.to_string();
    }
    output
}

fn partial_compare(buffer: &[u8], program: &[u8]) -> bool {
    if buffer.len() > program.len() {
        false
    } else {
        for (b, p) in buffer.iter().zip(program.iter()) {
            if *b != *p {
                return false;
            }
        }
        true
    }
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &Machine) -> i64 {
    let mut i = 0;
    'out: loop {
        println!("i: {i}");
        let mut machine = input.clone();
        //let mut loop_detect = HashSet::new();
        machine.reg_a = i;
        while let Some(ins) = machine.next_instruction() {
            //if loop_detect.contains(&(machine.reg_cp, machine.reg_a, machine.reg_b, machine.reg_c)) {
            //    i += 1;
            //    continue 'out;
            //}
            //loop_detect.insert((machine.reg_cp, machine.reg_a, machine.reg_b, machine.reg_c));
            if !machine.do_instruction(ins) {
                break;
            }
            if !partial_compare(&machine.output_buffer, &machine.instruction_memory) {
                i += 1;
                continue 'out;
            }
        }
        if machine.output_buffer == machine.instruction_memory {
            break;
        }
        i += 1;
    }
    i
}

#[test]
fn test_2_1() {
    let input = generator("Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0");
    let result = solve_part2(&input);
    assert_eq!(117440, result);
}
