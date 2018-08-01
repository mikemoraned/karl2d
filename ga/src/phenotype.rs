use rsgenetic::pheno::*;

const TARGET: i32 = 100;

#[derive(Copy, Clone, Debug)]
pub struct MyPheno {
    pub x: i32,
    pub y: i32,
}

impl Phenotype<i32> for MyPheno {
    // How fit is this individual?
    fn fitness(&self) -> i32 {
        TARGET - (self.x + self.y)
    }

    // Have two individuals create a new individual
    fn crossover(&self, other: &MyPheno) -> MyPheno {
        MyPheno {
            x: self.x,
            y: other.y,
        }
    }

    // Mutate an individual, changing its state
    fn mutate(&self) -> MyPheno {
        MyPheno {
            x: self.x + 1,
            y: self.y - 1,
        }
    }
}