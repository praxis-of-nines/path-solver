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

pub const NODE_COUNT: usize = 30;