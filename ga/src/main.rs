extern crate rsgenetic;

mod phenotype;

use rsgenetic::sim::*;
use rsgenetic::sim::seq::Simulator;
use rsgenetic::sim::select::*;

use phenotype::MyPheno;

fn main() {
    println!("Hello, world!");

    let mut population = (0..100).map(|i| MyPheno { x: i, y: 100 - i }).collect();
    let mut s = Simulator::builder(&mut population)
                    .set_selector(Box::new(StochasticSelector::new(10)))
                    .set_max_iters(50)
                    .build();
    s.run();
    let result = s.get().unwrap(); // The best individual

    println!("{:?}", result);
}
