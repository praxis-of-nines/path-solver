//! GA 
//! 
//! Implementation of the genetic algorithm which evolves the population. A new population entry is found
//! by first randomly merging 2 tours and then applying a mutation to this child together and then adding
//! a few mutations to create new genetic material.

extern crate rand;
use rand::Rng;
use population::*;
use tour::*;

pub struct GA;

impl GA {
    // TODO: Move to config

    // Set higher to increase the amount of new mutations. Lower to weigh more emphasis on the parent coupling.
    // Recommended was 0.015
    const MUTATION_RATE: f32 = 0.015;
    // Consider POPULATION_SIZE when setting tournament size.  This is the random sampling size from the population
    // that will compete for crossover (breeding) rights.  A size of 1 is equivalent to random.  4 means there is no chance
    // of the 2 least fittest potential parents being chosen (life isn't fair. ask a lobster)
    const TOURNAMENT_SIZE: usize = 5;
    // Choices are: roulette (0) / tournament (1) (default)
    const CROSSOVER_ALGORITHM: usize = 0;

    /// Evolve the population once creating 1 new generation
    ///
    /// # Examples
    ///
    /// ```
    /// let rng = &mut rand::thread_rng();
    /// let mut population = Population::new();
    /// 
    /// population = GA::evolve_population(rng, population);
    /// ```
    pub fn evolve_population(rng: &mut rand::ThreadRng, pop: Population) -> Population {
        let mut new_population = Population::new();
        let mut tours = Vec::new();

        // Loop over the desired population size and create tours crossed-over from
        // strong samples from current population
        for _ in 0..POP_COUNT {
            if GA::CROSSOVER_ALGORITHM == 0 {
                let random_value1: f32 = rng.gen::<f32>();
                let random_value2: f32 = rng.gen::<f32>();

                let parent1 = pop.get_tour(pop.get_tour_index_roulette(random_value1));
                let parent2 = pop.get_tour(pop.get_tour_index_roulette(random_value2));

                let child: Tour = GA::crossover(rng, &parent1, &parent2);
                tours.push(child);
            } else {
                let parent_index1 = pop.get_tour_index_tournament(GA::TOURNAMENT_SIZE);
                let parent_index2 = pop.get_tour_index_tournament(GA::TOURNAMENT_SIZE);

                let parent1 = pop.get_tour(parent_index1);
                let parent2 = pop.get_tour(parent_index2);

                let child: Tour = GA::crossover(rng, &parent1, &parent2);
                tours.push(child);
            }
        }

        new_population.initialize(tours);

        // Mutate the new population a bit to add some new genetic material
        for i in 0..POP_COUNT {
            GA::mutate(rng, new_population.get_tour_mut(i));
        }

        new_population.initialize_fitness();

        new_population
    }

    fn crossover(rng: &mut rand::ThreadRng, parent1: &Tour, parent2: &Tour) -> Tour {
        let mut child: Tour = Tour::new(parent1.get_tour_len());

        // Get start and end sub tour positions for parent1's tour
        let start_pos: usize = rng.gen_range(0, parent1.get_tour_len() as usize);
        let end_pos: usize = rng.gen_range(0, parent1.get_tour_len() as usize);

        // Loop and add the sub tour from parent1 to our child
        for i in 0..parent1.get_tour_len() as usize {
            // If our start position is less than the end position
            if start_pos < end_pos && i > start_pos && i < end_pos {
                child.set_node(i, parent1.get_node(i));

              // If our start position is larger
            } else if start_pos > end_pos { 
                if !(i < start_pos && i > end_pos) {
                    child.set_node(i, parent1.get_node(i));
                }
            }
        }

        // Loop through parent2's node tour
        for i in 0..parent1.get_tour_len() as usize {
            // If child doesn't have the node add it
            if !child.contains_node(parent2.get_node(i)) {
                // Loop to find a spare position in the child's tour
                for j in 0..parent2.get_tour_len() as usize {
                    // Spare position found, add node
                    if child.get_node(j).x == -1 {
                        child.set_node(j, parent2.get_node(i));
                        break;
                    }
                }
            }
        }

        child
    }

    fn mutate(rng: &mut rand::ThreadRng, tour: &mut Tour) {
        // Loop through tour
        for tour_pos1 in 0..tour.get_tour_len() as usize {
            // Apply mutation rate
            if rng.gen::<f32>() < GA::MUTATION_RATE {
                // Get a second random position in the tour
                let tour_pos2: usize = 0; //random.Next(NodeList.NodeCount);

                if tour_pos2 != tour_pos1 {
                    // Get the cities at target position in tour
                    let node1 = tour.get_node(tour_pos1);
                    let node2 = tour.get_node(tour_pos2);

                    // Swap them around
                    tour.set_node(tour_pos2, node1);
                    tour.set_node(tour_pos1, node2);
                }
            }
        }
    }
}