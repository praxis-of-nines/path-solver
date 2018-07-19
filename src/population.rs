//! Population
//!
//! Maintains a group of solutions (tours) and the functionality related to them.
//! Allows the selection of tours based on fittness of tours.

extern crate rand;

use tour::*;
use node::*;

use rand::Rng;


pub struct Population {
    tours: Vec<Tour>,
    total_fitness: f32,
}

pub const POP_COUNT: usize = 100;


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
            let mut new_tour = Tour::new(node_list.len());
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

    /// Get a set of randomly selected indices from the population of tours
    pub fn get_random_tour_indices(&self, amount: usize) -> Vec<usize> {
        let mut tour_index:Vec<usize> = Vec::new();

        for _i in 0..amount {
            let num = rand::thread_rng().gen_range(0, POP_COUNT - 1);

            tour_index.push(num);
        }

        tour_index
    }

    /// Roulette Selection
    ///
    /// Returns a psuedo random tour where greater weight is awarded tours with greater fitness. This is
    /// the method ensuring that over time the population has a greater chance to evolve rather than devolve. 
    ///
    /// # Examples
    ///
    /// ```
    /// let random_value1: f32 = rng.gen::<f32>();
    /// let random_value2: f32 = rng.gen::<f32>();
    ///
    /// let parent1 = pop.get_tour(pop.get_tour_index_roulette(random_value1));
    /// let parent2 = pop.get_tour(pop.get_tour_index_roulette(random_value2));
    /// ...crossover
    /// ```
    pub fn get_tour_index_roulette(&self, random_value: f32) -> usize {
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

    /// Tournament Selection
    ///
    /// Retrieves a random subset of the population and compares there fitness to return the top 2 parents from
    /// the group.
    ///
    /// # Examples
    ///
    /// ```
    /// let parent1 = pop.get_tour_index_tournament(4);
    /// let parent2 = pop.get_tour_index_tournament(4);
    /// let tour1 = pop.get_tour(parent1);
    /// let tour2 = pop.get_tour(parent2);
    /// ...crossover
    /// ```
    pub fn get_tour_index_tournament(&self, tournament_size: usize) -> usize { 

        let tour_index:Vec<usize> = self.get_random_tour_indices(tournament_size);

        let mut result: usize       = 0;
        let mut result_fitness: f32 = 0.0;

        for i in 0..tour_index.len() {
            if self.tours[tour_index[i]].fitness > result_fitness {
                result         = tour_index[i];
                result_fitness = self.tours[tour_index[i]].fitness;
            }
        }

        result
    }
}