use crate::beach::{Beach, self};
use crate::prey::{Algae, Clam, Minnow, Shrimp, self};
use crate::reef::{Reef, self};
use std::cell::RefCell;
use std::rc::Rc;
use std::slice::Iter;

#[derive(Debug)]
pub struct Ocean {
    // TODO: Fill in fields here.
    beaches: Vec<Beach>,
    reefs: Vec<Rc<RefCell<Reef>>>,


}

impl Ocean {
    pub fn new() -> Ocean {
        //unimplemented!();
        Ocean { 
            beaches: Vec::new(),
            reefs: Vec::new(),
        }
    }

    pub fn add_beach(&mut self, beach: Beach) {
        //unimplemented!();
        self.beaches.push(beach);
    }

    pub fn beaches(&self) -> Iter<Beach> {
        //unimplemented!();
        self.beaches.iter()
    }

    pub fn reefs(&self) -> Iter<Rc<RefCell<Reef>>> {
        //unimplemented!();
        self.reefs.iter()
    }

    /**
     * Generate a reef with the specified number of each concrete type of prey, and then add it to the ocean.
     *   - Minnows should have a speed of 25.
     *   - Shrimp should have an energy of 1.
     *
     * Returns a reference to the newly created reef.
     */
    pub fn generate_reef(
        &mut self,
        n_minnows: u32,
        n_shrimp: u32,
        n_clams: u32,
        n_algae: u32,
    ) -> Rc<RefCell<Reef>> {
        //unimplemented!();
        let mut reef_o =  Reef::new();
        for i in 0..n_algae {
            let mut preyy:Box<dyn prey::Prey> = Box::new(Algae::new());
            reef_o.add_prey(preyy);
        }
        for i in 0..n_minnows {
            let mut preyy:Box<dyn prey::Prey> = Box::new(Minnow::new(25));
            reef_o.add_prey(preyy);
        }
        for i in 0..n_shrimp {
            let mut preyy:Box<dyn prey::Prey> = Box::new(Shrimp::new(1));
            reef_o.add_prey(preyy);
        }
        for i in 0..n_clams {
            let mut preyy:Box<dyn prey::Prey> = Box::new(Clam::new());
            reef_o.add_prey(preyy);
        }
        Rc::new(RefCell::new(Reef::new()));
        let mut reef = Rc::new(RefCell::new(reef_o));
        let reef_copy = reef.clone();
        self.reefs.push(reef);
        reef_copy

    }
}
