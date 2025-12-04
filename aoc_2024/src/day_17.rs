// https://adventofcode.com/2024/day/17

type LiteralOperand = u64;
type ComboOperand = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Adv(ComboOperand),
    Bxl(LiteralOperand),
    Bst(ComboOperand),
    Jnz(LiteralOperand),
    Bxc,
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand)
}

const REGISTER_A:usize = 0;
const REGISTER_B:usize = 1;
const REGISTER_C:usize = 2;

fn run_program(program: &[Instruction], registers: &mut [u64; 3]) -> Vec<u64> {
    let mut pc = 0;
    let mut output = vec![];
    loop {
        if pc >= program.len() {
            break;
        }
        let instruction = program[pc];
        match instruction {
            Instruction::Adv(combo) => {
                let value = get_value_from_combo(combo, registers);
                registers[REGISTER_A] /= 2u64.pow(value as u32);
                pc += 1;
            },
            Instruction::Bxl(literal) => {
                registers[REGISTER_B] ^= literal;
                pc += 1;
            },
            Instruction::Bst(combo) => {
                let value = get_value_from_combo(combo, registers);
                registers[REGISTER_B] = value % 8;
                pc += 1;
            },
            Instruction::Jnz(literal) => {
                if registers[REGISTER_A] == 0 {
                    pc += 1;
                    
                    continue;
                }                
                
                pc = literal as usize;
            },
            Instruction::Bxc => {
                registers[REGISTER_B] ^= registers[REGISTER_C];
                pc += 1;
            },
            Instruction::Out(combo) => {
                let value = get_value_from_combo(combo, registers);
                output.push(value % 8);
                pc += 1;
            },
            Instruction::Bdv(combo) => {
                let value = get_value_from_combo(combo, registers);
                registers[REGISTER_B] = registers[REGISTER_A] / 2_u64.pow(value as u32);
                pc += 1;
            },
            Instruction::Cdv(combo) => {
                let value = get_value_from_combo(combo, registers);
                registers[REGISTER_C] = registers[REGISTER_A] / 2_u64.pow(value as u32);
                pc += 1;
            }
        }
    }

    output
}

fn get_value_from_combo(operand: ComboOperand, registers: &[u64; 3]) -> u64 {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => registers[REGISTER_A],
        5 => registers[REGISTER_B],
        6 => registers[REGISTER_C],
        _ => panic!("Invalid register")
    }
}

pub fn solve_pt1(data: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let mut registers:[u64; 3] = [0; 3];

    let mut data_iter = data.lines();

    let reg_a:u64 = data_iter
    .next().unwrap()
    .strip_prefix("Register A: ").unwrap()
    .parse()?;

    registers[REGISTER_A] = reg_a;

    let reg_b:u64 = data_iter
    .next().unwrap()
    .strip_prefix("Register B: ").unwrap()
    .parse()?;
    registers[REGISTER_B] = reg_b;

    let reg_c:u64 = data_iter
    .next().unwrap()
    .strip_prefix("Register C: ").unwrap()
    .parse()?;

    registers[REGISTER_C] = reg_c;

    data_iter.next();

    let instructions_as_numbers:Vec<u64> = data_iter
    .next().ok_or("No more data")?
    .strip_prefix("Program: ").ok_or("Parse error")?
    .split(",")
    .map(|x| x.parse().unwrap())
    .collect();

    let mut program: Vec<Instruction> = vec![];

    for i in (0..instructions_as_numbers.len()).step_by(2) {
        let instr = instructions_as_numbers[i];
        let operand = instructions_as_numbers[i+1];
        let instruction = match instr {
            0 => Instruction::Adv(operand),
            1 => Instruction::Bxl(operand),
            2 => Instruction::Bst(operand),
            3 => Instruction::Jnz(operand),
            4 => Instruction::Bxc,
            5 => Instruction::Out(operand),
            6 => Instruction::Bdv(operand),
            7 => Instruction::Cdv(operand),
            _ => panic!("Invalid instruction")
        };
        program.push(instruction);
    }

    let output = run_program(&program, &mut registers);

    println!("{:?}", output);
    
    Ok(())
}

pub fn solve_pt2(_: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let nums:Vec<u64> = [2,4,1,7,7,5,0,3,4,0,1,7,5,5,3,0].iter().copied().rev().collect();
 
    let res = dfs_starting_with_lowest(0, &nums);
    println!("{:?}", res);
    Ok(())
}


fn dfs_starting_with_lowest(a:u64, nums: &[u64]) -> Option<u64> {
    
    if nums.is_empty() {
        return Some(a);
    }

    let num_to_find = nums[0];
    for i in 0..8 {
        let mut b = i; // Represents 3 bits from A
        b ^= 7; 
        
        let c = (a+i) >> b;
        b ^= c;
        b ^= 7;
        
        if (b % 8) == num_to_find {
            let a = a + i;
            if nums.len() == 1 {
                return Some(a);
            }
            let a = a << 3;
            let res = dfs_starting_with_lowest(a, &nums[1..]);

            if res.is_some() {
                return res;
            }
        }
    }
    
    Option::None
}


#[cfg(test)]
mod tests {
    use super::{solve_pt1, solve_pt2};


    #[test]
    fn pt1_example_1() {
        let data = 
r"Register A: 8
Register B: 3
Register C: 4

Program: 0,2,5,4,5,5,5,6";
        _ = solve_pt1(data);
    }

    #[test]
    fn pt1_example_2() {
        let data = 
r"Register A: 32
Register B: 0
Register C: 0

Program: 5,4,0,1,3,0";
        _ = solve_pt1(data);
    }

    #[test]
    fn test_solve_pt1() {
        let data = include_str!("day_17_input");
        let result = solve_pt1(data);
        println!("{:?}", result);
    }

    #[test]
    fn test_solve_pt2() {
        let data = include_str!("day_17_input");
        let result = solve_pt2(data);
        println!("{:?}", result);
    }

}
