extern crate genevo;
extern crate rand;

use genevo::operator::prelude::*;
use genevo::population::ValueEncodedGenomeBuilder;
use genevo::prelude::*;
use genevo::types::fmt::Display;

use std::cmp;

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
struct Goal {
    target_length : usize,
}

#[derive(Clone, Copy)]
struct FitnessCalc<'a> {
    goal: &'a Goal,
}

impl<'a> FitnessFunction<Genotype, usize> for FitnessCalc<'a> {
    fn fitness_of(&self, genome: &Genotype) -> usize {
        let length = genome.as_path().iter()
            .fold(0, |acc, e| acc + match e {
                PathElement::Forward => 1,
                _ => 0
            });
        cmp::min(self.goal.target_length, length)
    }

    fn average(&self, fitness_values: &[usize]) -> usize {
        fitness_values.iter().sum::<usize>() / fitness_values.len()
    }

    fn highest_possible_fitness(&self) -> usize {
        self.goal.target_length
    }

    fn lowest_possible_fitness(&self) -> usize {
        0
    }
}

fn main() {
    println!("Hello, world!");

    let goal : Goal = Goal {
        target_length: 200,
    };

    let population_size = 100;

    let initial_population: Population<Genotype> = build_population()
        .with_genome_builder(ValueEncodedGenomeBuilder::new(goal.target_length, 0, 1))
        .of_size(population_size)
        .uniform_at_random();

    let fitness_calc = FitnessCalc { goal: &goal };

    let ga = genetic_algorithm()
        .with_evaluation(fitness_calc)
        .with_selection(MaximizeSelector::new(0.7, 2))
        .with_crossover(MultiPointCrossBreeder::new(goal.target_length / 6))
        .with_mutation(RandomValueMutator::new(0.01, 0, 1))
        .with_reinsertion(ElitistReinserter::new(
            fitness_calc,
            true,
            0.7,
        ))
        .with_initial_population(initial_population)
        .build();

    let mut sim = simulate(ga)
        .until(GenerationLimit::new(2000))
        .build();

    loop {
        let result = sim.step();
        match result {
            Ok(SimResult::Intermediate(step)) => {
                let best_solution = step.result.best_solution;
                println!("{:?}", best_solution.solution.genome.as_path());
            },
            Ok(SimResult::Final(step, processing_time, duration, stop_reason)) => {
                let best_solution = step.result.best_solution;
                println!("{}", stop_reason);
                println!("{:?}", best_solution.solution.genome.as_path());
                break;
            },
            Err(error) => {
                println!("{}", error.display());
                break;
            },
        }
    }
}
