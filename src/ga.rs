//! GA 
//! 
//! Implementation of the genetic algorithm which evolves the population. A new population entry is found
//! by first randomly merging 2 tours and then applying a mutation to this child together and then adding
//! a few mutations to create new genetic material.

extern crate rand;
use rand::Rng;
use population::*;
use tour::*;
use node::*;

pub struct GA;

impl GA {
    // Set higher to increase the amount of new mutations. Lower to weigh more emphasis on the parent coupling
    const MUTATION_RATE: f32 = 0.015;

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
            let random_value1: f32 = rng.gen::<f32>();
            let random_value2: f32 = rng.gen::<f32>();

            let parent1 = pop.get_tour(pop.get_random_tour(random_value1));
            let parent2 = pop.get_tour(pop.get_random_tour(random_value2));

            let child: Tour = GA::crossover(rng, &parent1, &parent2);
            tours.push(child);
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
        let mut child: Tour = Tour::new();

        // Get start and end sub tour positions for parent1's tour
        let start_pos: usize = rng.gen_range(0, NODE_COUNT);
        let end_pos: usize = rng.gen_range(0, NODE_COUNT);

        // Loop and add the sub tour from parent1 to our child
        for i in 0..NODE_COUNT {
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
        for i in 0..NODE_COUNT {
            // If child doesn't have the node add it
            if !child.contains_node(parent2.get_node(i)) {
                // Loop to find a spare position in the child's tour
                for j in 0..NODE_COUNT {
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
        // Loop through tour cities
        for tour_pos1 in 0..NODE_COUNT {
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