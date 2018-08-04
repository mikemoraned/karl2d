use num;

#[derive(Debug, Clone, PartialEq, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Instruction {
    Forward,
    Stay
}

impl Instruction {
    fn all_integer_values() -> Vec<u8> {
        let variants = vec![Instruction::Forward, Instruction::Stay];
        variants.iter().map(|v| num::ToPrimitive::to_u8(v).unwrap()).collect()
    }

    pub fn min_u8_value() -> u8 {
        *Instruction::all_integer_values().iter().min().unwrap()
    }
    
    pub fn max_u8_value() -> u8 {
        *Instruction::all_integer_values().iter().max().unwrap()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Turtle {
    pub x: usize
}

pub trait Application {
    fn apply_instructions(self: &Self, instructions: &Vec<Instruction>) -> Turtle;
    fn apply_instruction(self: &Self, instruction: &Instruction) -> Turtle;
}

impl Application for Turtle {
    fn apply_instructions(self: &Self, instructions: &Vec<Instruction>) -> Self {
        return instructions.iter()
            .fold(*self, |turtle, i| turtle.apply_instruction(&i) );
    }

    fn apply_instruction(self: &Self, instruction: &Instruction) -> Self {
        match instruction {
            Instruction::Forward => Self{ x: self.x + 1 },
            _ => *self
        }   
    }
}


