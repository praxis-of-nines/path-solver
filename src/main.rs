extern crate rand;
extern crate num;
extern crate chrono;

mod node;
mod tour;
mod population;
mod ga;

use std::env;

use node::*;
use population::*;
use ga::*;
use chrono::prelude::*;

fn main() {

    //  5’ x 10’ / 1524mm x 3048mm
    let mut node_list: Vec<Node> = Vec::new();

    let args: Vec<_> = env::args().collect();

    for i in 1..args.len() {
        let coord_pair: Vec<&str> = args[i].split(",").collect();


        node_list.push(Node { x: coord_pair[0].parse::<i32>().unwrap(), y: coord_pair[1].parse::<i32>().unwrap(), m: 'n' });
    }

    let rng = &mut rand::thread_rng();
    let mut population = Population::new();
    population.initialize_from_nodes(rng, &node_list);

    // fittest_tour borrows temporarily from population:
    {
        let fittest_tour = population.get_fittest();
        let run_time = fittest_tour.get_distance() as f64 / 3000.0 + (0.1 * node_list.len() as f64);
        println!("starting fittest {}", fittest_tour.get_distance());
        println!("starting run-time {:.2}", run_time);
        println!("amount nodes {}", fittest_tour.get_tour_len())
    }

    let start = Utc::now();

    for _ in 1..2000 {
        population = GA::evolve_population(rng, population);
    }

    let end = Utc::now();

    let fittest_tour = population.get_fittest();

    let run_time = fittest_tour.get_distance() as f64 / 3000.0 + (0.1 * node_list.len() as f64);
    
    //println!("fittest tour \n  {:?}", fittest_tour.get_nodes());
    println!("ending fittest {}", fittest_tour.get_distance());
    println!("ending run-time {:.2}", run_time);
    println!("Time to solve problem {:?} seconds", end.signed_duration_since(start).to_std().unwrap())
}
