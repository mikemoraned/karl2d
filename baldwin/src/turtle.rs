#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum PathElement {
    Forward,
    Stay
}

#[derive(Debug, Copy, Clone)]
pub struct Turtle {
    pub x: usize
}

pub type Path = Vec<PathElement>;

pub trait TurtleMovement {
    fn move_along_path(self: &Self, path: &Path) -> Turtle;
    fn move_along_element(self: &Self, element: &PathElement) -> Turtle;
}

impl TurtleMovement for Turtle {
    fn move_along_path(self: &Self, path: &Path) -> Self {
        return path.iter()
            .fold(*self, |turtle, element| turtle.move_along_element(&element) );
    }

    fn move_along_element(self: &Self, element: &PathElement) -> Self {
        match element {
            PathElement::Forward => Self{ x: self.x + 1 },
            _ => *self
        }   
    }
}


