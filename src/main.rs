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

    /*node_list.push(Node { x: 604, y: 2009, m: 'n' });
    node_list.push(Node { x: 180, y: 3004, m: 'n' });
    node_list.push(Node { x: 800, y: 1080, m: 'n' });
    node_list.push(Node { x: 140, y: 180, m: 'n' });
    node_list.push(Node { x: 20, y: 160, m: 'n' });
    node_list.push(Node { x: 1000, y: 1160, m: 'n' });
    node_list.push(Node { x: 1200, y: 2160, m: 'n' });
    node_list.push(Node { x: 1140, y: 140, m: 'n' });
    node_list.push(Node { x: 40, y: 120, m: 'n' });
    node_list.push(Node { x: 1000, y: 2125, m: 'n' });
    node_list.push(Node { x: 180, y: 2890, m: 'n' });
    node_list.push(Node { x: 60, y: 80, m: 'n' });
    node_list.push(Node { x: 120, y: 80, m: 'n' });
    node_list.push(Node { x: 120, y: 600, m: 'n' });
    node_list.push(Node { x: 290, y: 400, m: 'n' });
    node_list.push(Node { x: 100, y: 40, m: 'n' });
    node_list.push(Node { x: 200, y: 2440, m: 'n' });
    node_list.push(Node { x: 20, y: 20, m: 'n' });
    node_list.push(Node { x: 60, y: 20, m: 'n' });
    node_list.push(Node { x: 1200, y: 2090, m: 'n' });
    node_list.push(Node { x: 1250, y: 2455, m: 'n' });
    node_list.push(Node { x: 20, y: 280, m: 'n' });
    node_list.push(Node { x: 3, y: 2090, m: 'n' });
    node_list.push(Node { x: 300, y: 1240, m: 'n' });
    node_list.push(Node { x: 200, y: 86, m: 'n' });
    node_list.push(Node { x: 202, y: 1780, m: 'n' });
    node_list.push(Node { x: 130, y: 1202, m: 'n' });
    node_list.push(Node { x: 60, y: 60, m: 'n' });
    node_list.push(Node { x: 1234, y: 2345, m: 'n' });
    node_list.push(Node { x: 1, y: 1, m: 'n' });*/
    /*
    node_list.push(Node { x: 100,  y: 28,   m: 'n' });
    node_list.push(Node { x: 672,  y: 72,   m: 'n' });
    node_list.push(Node { x: 2220, y: 56,   m: 'n' });
    node_list.push(Node { x: 1096, y: 444,  m: 'n' });
    node_list.push(Node { x: 1084, y: 864,  m: 'n' });
    node_list.push(Node { x: 2228, y: 944,  m: 'n' });
    node_list.push(Node { x: 2528, y: 1052, m: 'n' });
    node_list.push(Node { x: 2164, y: 1212, m: 'n' });
    node_list.push(Node { x: 2276, y: 1468, m: 'n' });
    node_list.push(Node { x: 696,  y: 1544, m: 'n' });
    node_list.push(Node { x: 1020, y: 1812, m: 'n' });
    node_list.push(Node { x: 8,    y: 2136, m: 'n' });
    node_list.push(Node { x: 612,  y: 2084, m: 'n' });
    node_list.push(Node { x: 732,  y: 2320, m: 'n' });
    node_list.push(Node { x: 896,  y: 2500, m: 'n' });
    node_list.push(Node { x: 2668, y: 2608, m: 'n' });*/

    let rng = &mut rand::thread_rng();
    let mut population = Population::new();
    population.initialize_from_nodes(rng, &node_list);

    // fittest_tour borrows temporarily from population:
    {
        let fittest_tour = population.get_fittest();
        let run_time = fittest_tour.get_distance() as f64 / 3000.0 + (0.1 * node_list.len() as f64);
        println!("starting fittest {}", fittest_tour.get_distance());
        println!("starting run-time {:.2}", run_time)
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
