use crate::prey::{Prey, self};

// VecDeque is Rust's implementation of a double-ended queue, and
// is used only if we only need to use it in a single-ended manner.
use std::collections::vec_deque::{Iter, VecDeque};

#[derive(Debug)]
pub struct Reef {
    prey: VecDeque<Box<dyn Prey>>,
}

impl Reef {
    pub fn new() -> Self {
        Reef { prey: (VecDeque::new()) }
    }

    pub fn prey(&self) -> Iter<Box<dyn Prey>> {
        //unimplemented!();
        self.prey.iter()
    }

    pub fn population(&self) -> usize {
        //unimplemented!();
        self.prey.len()
    }

    /**
     * Adds a prey to the reef.
     *
     * This function takes ownership of the boxed prey.
     */
    pub fn add_prey(&mut self, prey: Box<dyn Prey>) {
        //unimplemented!();
        self.prey.push_back(prey);
    }

    /**
     * Returns the next available prey.
     *
     * The callee of this function receives ownership of the boxed prey.
     */
    pub fn take_prey(&mut self) -> Option<Box<dyn Prey>> {
        //unimplemented!();
        if self.prey.len() == 0{
            None
        } else {
            self.prey.pop_front()
        }
        
    }
}
