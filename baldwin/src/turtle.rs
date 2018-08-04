#[derive(Debug, Clone, PartialEq, PartialOrd, FromPrimitive)]
pub enum Instruction {
    Forward,
    Stay
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


