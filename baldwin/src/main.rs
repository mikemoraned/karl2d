extern crate genevo;
extern crate rand;
extern crate num;
#[macro_use] extern crate num_derive;

use genevo::operator::prelude::*;
use genevo::population::ValueEncodedGenomeBuilder;
use genevo::prelude::*;
use genevo::types::fmt::Display;

use std::cmp;

mod turtle;

use turtle::*;

type Genotype = Vec<u8>;
type Phenotype = Vec<Instruction>;

trait AsPhenotype {
    fn as_instructons(&self) -> Phenotype;
}

impl AsPhenotype for Genotype {
    fn as_instructons(&self) -> Phenotype {
        fn as_instruction(integer: &u8) -> Instruction {
            match num::FromPrimitive::from_u8(*integer) {
                Some(instruction) => instruction,
                None => panic!("integer {} not mappable")
            }
        }

        self.iter().map(|i| as_instruction(i)).collect()
    }
}

#[derive(Copy, Clone)]
struct Goal {
    target_length : usize,
}

#[derive(Clone, Copy)]
struct FitnessCalc {
    goal: Goal,
}

impl FitnessFunction<Genotype, usize> for FitnessCalc {
    fn fitness_of(&self, genome: &Genotype) -> usize {
        let instructions = genome.as_instructons();
        let start = turtle::Turtle{ x: 0 };
        let end = start.apply_instructions(&instructions); 
        let length = end.x;
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
        .with_genome_builder(ValueEncodedGenomeBuilder::new(goal.target_length, Instruction::min_u8_value(), Instruction::max_u8_value() + 1))
        .of_size(population_size)
        .uniform_at_random();

    let fitness_calc = FitnessCalc { goal };

    let ga = genetic_algorithm()
        .with_evaluation(fitness_calc)
        .with_selection(MaximizeSelector::new(0.7, 2))
        .with_crossover(MultiPointCrossBreeder::new(goal.target_length / 6))
        .with_mutation(RandomValueMutator::new(0.01, Instruction::min_u8_value(), Instruction::max_u8_value() + 1))
        .with_reinsertion(ElitistReinserter::new(
            fitness_calc,
            true,
            0.7,
        ))
        .with_initial_population(initial_population)
        .build();

    let mut sim = simulate(ga)
        .until(or(
            GenerationLimit::new(2000),
            FitnessLimit::new(fitness_calc.highest_possible_fitness())))
        .build();

    loop {
        let result = sim.step();
        match result {
            Ok(SimResult::Intermediate(step)) => {
                let evaluated_population = step.result.evaluated_population;
                let best_solution = step.result.best_solution;
                println!(
                    "Step: generation: {}, average_fitness: {}, \
                     best fitness: {}, duration: {}, processing_time: {}",
                    step.iteration,
                    evaluated_population.average_fitness(),
                    best_solution.solution.fitness,
                    step.duration.fmt(),
                    step.processing_time.fmt()
                );
                println!("{:?}", best_solution.solution.genome.as_instructons());
            },
            Ok(SimResult::Final(step, _, _, stop_reason)) => {
                let best_solution = step.result.best_solution;
                println!("{}", stop_reason);
                println!("{:?}", best_solution.solution.genome.as_instructons());
                break;
            },
            Err(error) => {
                println!("{}", error.display());
                break;
            },
        }
    }
}
