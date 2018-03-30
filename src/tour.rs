extern crate rand;
extern crate num;
use rand::Rng;
use node::*;

pub struct Tour {
    tour: [Node; NODE_COUNT],
    pub fitness: f32,
    pub relative_fitness: f32,
    pub amplified_fitness: f32,
}

const AMPLIFY_FACTOR: f32 = 2f32;

//! Tour
//!
//! Represents a single path (solution) which is a vector of nodes ordered by traversal
//! 
impl Tour {
    pub fn new() -> Tour {
        Tour { 
            fitness: 0.0,
            relative_fitness: 0.0,
            amplified_fitness: 0.0,
            tour: [Node::default(); NODE_COUNT],
        }
    }

    /// Generate the tour based on the nodes.  Copies the node and then shuffles to create a randomly
    /// generated tour.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut new_tour = Tour::new();
    /// new_tour.generate_individual(rng, &node_list);
    /// ```
    pub fn generate_individual(&mut self, rng: &mut rand::ThreadRng, node_list: &Vec<Node>) {
        assert_eq!(node_list.len(), NODE_COUNT);

        // copy nodes in original sequence
        for i in 0..NODE_COUNT {
            self.tour[i] = node_list[i];
        }

        // shuffle to create new sequence
        for _ in 0..100 {
            for j in 0..NODE_COUNT {
                let random_index: i32 = rng.gen_range(0, NODE_COUNT as i32);
                if random_index != j as i32
                {
                    // swap the node at j with the node at random_index:
                    let save_node = self.get_node(j);
                    let random_node = self.get_node(random_index as usize);
                    self.tour[j] = random_node;
                    self.tour[random_index as usize] = save_node;
                }
            }
        }
    }

    pub fn get_node(&self, index: usize) -> Node {
        self.tour[index]
    }

    pub fn set_node(&mut self, index: usize, node: Node) {
        self.fitness = 0.0;
        self.relative_fitness = 0.0;
        self.amplified_fitness = 0.0;
        self.tour[index] = node;
    }

    pub fn contains_node(&self, find_node: Node) -> bool {
        for tour in &self.tour {
            if tour.x == find_node.x && tour.y == find_node.y {
                return true;
            }
        }
        false
    }

    pub fn set_fitness(&mut self) {
        let distance: f32 = self.get_distance() as f32;
        
        assert!(distance > 0.0001);

        self.fitness = (1f32 / distance) * 100f32;
    }

    pub fn set_relative_fitness(&mut self, total_fitness: f32, average_fitness: f32) {
        assert!(total_fitness > 0.0001);

        self.relative_fitness = self.fitness / total_fitness;
        self.amplified_fitness = (self.fitness + ((self.fitness - average_fitness) * AMPLIFY_FACTOR)) / total_fitness;
    }

    fn distance_to(&self, from_node: Node, to_node: Node) -> f32 {
        let x_distance: i32 = num::abs(from_node.x - to_node.x);
        let y_distance: i32 = num::abs(from_node.y - to_node.y);
        
        // Pythagorean calculation
        let sums_squared: i32 = (x_distance * x_distance) + (y_distance * y_distance);
        let distance: f32 = num::Float::sqrt(sums_squared as f32);

        distance
    }

    pub fn get_distance(&self) -> i32 {
        let mut tour_distance: i32 = 0;
        
        for i in 0..NODE_COUNT {
            let from_node = self.tour[i];
            let destination_node =
                // check we're not on our tour's last node, if we are set our
                // tour's final destination node to our starting node
                if i + 1 < NODE_COUNT {
                    self.tour[i + 1]
                } else {
                    self.tour[0]
                };

            tour_distance += self.distance_to(from_node, destination_node) as i32;
        }

        tour_distance
    }
}