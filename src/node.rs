//! Node 
//! 
//! A structure to represent a 'stop' on a tour, the primary structure of the program of which must
//! be traversed.  Should be concurrent ready (copy, clone).

use std::fmt::Debug;
use std::fmt::Result;
use std::fmt::Formatter;

#[derive(Copy, Clone)]
pub struct Node {
    pub x: i32,
    pub y: i32,
    pub m: char
}

impl Default for Node {
    #[inline]
    fn default() -> Node {
        Node { x: -1, y: -1, m: 'n' }
    }
}

impl Debug for Node {
	fn fmt(&self, f: &mut Formatter) -> Result {
      write!(f, "[{:?}, {:?}, {:?}]\n", self.x, self.y, self.m)
	}
}