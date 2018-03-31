//! Population
//!
//! Maintains a group of solutions (tours) and the functionality related to them.
//! Allows the selection of tours based on fittness of tours.

extern crate rand;
use tour::*;
use node::*;


pub struct Population {
    tours: Vec<Tour>,
    total_fitness: f32,
}

pub const POP_COUNT: usize = 50;


impl Population {
    /// Create a new empty population
    ///
    /// # Examples
    ///
    /// ```
    /// let mut new_population = Population::new();
    /// ```
    pub fn new() -> Population {
        Population {
            total_fitness: 0.0,
            tours: Vec::new(),
        }
    }

    /// Use-once function for initializing a group of tours based on the input node node_list
    ///
    /// # Examples
    ///
    /// ```
    /// let rng = &mut rand::thread_rng();
    /// let mut population = Population::new();
    /// population.initialize_from_nodes(rng, &node_list);
    /// ```
    pub fn initialize_from_nodes(&mut self, rng: &mut rand::ThreadRng, node_list: &Vec<Node>) {
        assert_eq!(self.tours.len(), 0);
        
        for _ in 0..POP_COUNT {
            let mut new_tour = Tour::new();
            new_tour.generate_individual(rng, &node_list);
            self.tours.push(new_tour);
        }
        
        self.initialize_fitness();
    }

    /// Initialize a population with a set of tours
    pub fn initialize(&mut self, tours: Vec<Tour>) {
        assert_eq!(tours.len(), POP_COUNT);
        assert_eq!(self.tours.len(), 0);
        
        for tour in tours {
            self.tours.push(tour);
        }
    }

    /// Initialize the fitness to default for the set of tours
    pub fn initialize_fitness(&mut self) {
        self.total_fitness = 0.0;

        for i in 0..POP_COUNT {
            self.tours[i].set_fitness();
            self.total_fitness += self.tours[i].fitness;
        }

        for i in 0..POP_COUNT {
            self.tours[i].set_relative_fitness(self.total_fitness, self.total_fitness / (POP_COUNT as f32));
        }
    }

    /// Return a specific tour reference
    pub fn get_tour(&self, index: usize) -> &Tour {
        &self.tours[index]
    }

    /// Return a specific tour in mutable form
    pub fn get_tour_mut(&mut self, index: usize) -> &mut Tour {
        &mut self.tours[index]
    }

    /// Return the fittest (shortest path) tour 
    pub fn get_fittest(&mut self) -> &Tour {
        let mut max_tour = &self.tours[0];
        for i in 0..POP_COUNT {
            if self.tours[i].fitness > max_tour.fitness {
                max_tour = &self.tours[i];
            }
        }
        max_tour
    }

    /// Returns a psuedo random tour where greater weight is awarded tours with greater fitness. This is
    /// the method ensuring that over time the population evolves rather than devolves.
    ///
    /// # Examples
    ///
    /// ```
    /// let random_value1: f32 = rng.gen::<f32>();
    /// let random_value2: f32 = rng.gen::<f32>();
    ///
    /// let parent1 = pop.get_tour(pop.get_random_tour(random_value1));
    /// let parent2 = pop.get_tour(pop.get_random_tour(random_value2));
    /// ...crossover
    /// ```
    pub fn get_random_tour(&self, random_value: f32) -> usize {
        let mut relative_total: f32 = 0.0;
        let mut result: usize = 0;

        if self.total_fitness > 0.0 {
            // randomValue is a number between 0 and 1.0
            // relativeTotal increments from 0 to 1.0
            for i in 0..POP_COUNT {
                if random_value < relative_total + self.tours[i].amplified_fitness {
                    result = i;
                    break;
                } else {
                    relative_total = relative_total + self.tours[i].amplified_fitness;
                }
            }
        }

        result
    }
}