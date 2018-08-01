extern crate genevo;
extern crate rand;

use genevo::operator::prelude::*;
use genevo::population::ValueEncodedGenomeBuilder;
use genevo::prelude::*;
use genevo::types::fmt::Display;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum PathElement {
    Forward,
    Stay
}

type Genotype = Vec<u8>;
type Path = Vec<PathElement>;
type Phenotype = Path;

trait AsPhenotype {
    fn as_path(&self) -> Phenotype;
}

impl AsPhenotype for Genotype {
    fn as_path(&self) -> Phenotype {
        fn as_path_element(i: &u8) -> PathElement {
            match i {
                0 => PathElement::Forward,
                _ => PathElement::Stay
            }
        }

        self.iter().map(|i| as_path_element(i)).collect()
    }
}

#[derive(Clone)]
struct FitnessCalc {
    goal_distance : usize,
}

impl FitnessFunction<Genotype, usize> for FitnessCalc {
    fn fitness_of(&self, genome: &Genotype) -> usize {
        0
    }

    fn average(&self, fitness_values: &[usize]) -> usize {
        fitness_values.iter().sum::<usize>() / fitness_values.len()
    }

    fn highest_possible_fitness(&self) -> usize {
        100_00
    }

    fn lowest_possible_fitness(&self) -> usize {
        0
    }
}

fn main() {
    println!("Hello, world!");

    let target_length = 200;

    let population_size = 100;

    let initial_population: Population<Genotype> = build_population()
        .with_genome_builder(ValueEncodedGenomeBuilder::new(target_length, 0, 1))
        .of_size(population_size)
        .uniform_at_random();
}
