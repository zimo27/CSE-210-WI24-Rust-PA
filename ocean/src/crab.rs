use crate::color::Color;
use crate::cookbook::{Cookbook, Recipe};
use crate::diet::{Diet, self};
use crate::prey::Prey;
use crate::reef::Reef;
use std::cell::RefCell;
use std::rc::Rc;
use std::vec;

#[derive(Debug)]
pub struct Crab {
    // TODO: Add fields here (some in part 1, some in part 2)
    name: String,
    speed: u32,
    color:  Color,
    diet: Diet,
    reefs: Vec<Rc<RefCell<Reef>>>

}

// Do NOT implement Copy for Crab.
impl Crab {
    pub fn new(name: String, speed: u32, color: Color, diet: Diet) -> Crab {
        //unimplemented!();
        Crab {
            name : name,
            speed : speed,
            color : color,
            diet : diet,
            reefs: Vec::new()
        }
    }

    pub fn name(&self) -> &str {
        //unimplemented!();
        &self.name
    }

    pub fn speed(&self) -> u32 {
        //unimplemented!();
        self.speed
    }

    pub fn color(&self) -> &Color {
        //unimplemented!();
        &self.color
    }

    pub fn diet(&self) -> Diet {
        //unimplemented!();
        // why Diet need no &???
        self.diet
    }

    // PART 2 BELOW
    // ------------

    /**
     * Have this crab discover a new reef, adding it to its list of reefs.
     */
    pub fn discover_reef(&mut self, reef: Rc<RefCell<Reef>>) {
        //unimplemented!();
        self.reefs.push(reef);
    }

    /**
     * Returns Some prey from one of the reefs this crab feeds from,
     * and the index of that reef in self.reefs if able to find Some prey
     * using the `take_prey` method of Reef.
     *
     * If `take_prey` returns None, try the next reef. Try each reef only once.
     *
     * If all reefs are empty, or this crab has no reefs, return None.
     */
    fn catch_prey(&mut self) -> Option<(Box<dyn Prey>, usize)> {
        //unimplemented!();
        if self.reefs.len() == 0 {
            return None;
        }
        let mut index:usize = 0;
        for re in self.reefs.iter() {
            let mut r = re.borrow_mut();
            if r.population() == 0 {
                continue;
            } else {
                return Some((r.take_prey().unwrap(),index))
            }
            index+=1;
        
        }

        None
    }

    /**
     * Releases the given prey back into the reef at the given index.
     */
    fn release_prey(&mut self, prey: Box<dyn Prey>, reef_index: usize) {
        //unimplemented!();
        let mut r = self.reefs[reef_index].borrow_mut();
        r.add_prey(prey);
    }

    /**
     * Have this crab go hunting.
     *
     * A crab will keep trying to catch prey until it succeeds,
     * or runs out of remaining prey to try to catch.
     *
     * You should keep track of all escaped prey in a local.
     *
     * Once you have finished hunting, release all escaped prey back into
     * the reefs from whence they came _before_ you return if prey was caught.
     *
     * Your algorithm might look something like this pseudocode. The challenge
     * in this task is not intended to be in figuring out the algorithm, but
     * in figuring out _how to write it in Rust_ using what you've learned so far.
     *
     * ```text
     *     there are no escaped prey yet
     *     prey has not been caught
     *     while prey can be caught
     *       if prey escapes
     *         mark the prey as escaped
     *         try again
     *     
     *       if prey is not edible by this crab
     *         mark the prey as escaped
     *         try again
     *       
     *       prey has been caught
     *       stop trying
     *     
     *     release each escaped prey back to its reef
     *     was prey caught?
     * ```
     *
     * Note: this pseudocode reads like a terrible poem.
     */
    pub fn hunt(&mut self) -> bool {
        //unimplemented!();
        if self.reefs.len() == 0 {
            return false;
        }
        let mut escaped:Vec<(Box<dyn Prey>, usize)> = Vec::new();
        let mut catched_prey:Option<(Box<dyn Prey>, usize)> = self.catch_prey();
        let mut success:u8 = 0;
        while !catched_prey.is_none() {
            let mut preyy:(Box<dyn Prey>, usize) = catched_prey.unwrap();
            // if run away
            if preyy.0.try_escape(&self) {
                escaped.push(preyy);
            // if not edible
            } else if preyy.0.diet() != self.diet() {
                escaped.push(preyy);
            // if caught the prey
            } else {
                //return true;
                success = 1;
            }
            catched_prey = self.catch_prey();
        }
        for da_prey in escaped.drain(..) {
            self.release_prey(da_prey.0, da_prey.1)
        }
        // return all escaped preys
        if success==1 {
            return true;
        } else {
            return false;
        }
        
    }

    pub fn breed(&self, crab_b:&Crab, name:String) -> Crab{ 
        Crab { name: (name), speed: (1), color: (Color::cross(self.color(),crab_b.color())), diet: (Diet::random_diet()) , reefs:Vec::new()}
    }

    /**
     * Returns Some of any recipe from the given cookbook that matches the crab's diet
     * preferences, or None if no such recipe exists.
     *
     * IMPORTANT: you will need to add lifetime parameters to this function. It is
     * up to you to figure out which ones and where. Do not make any other changes
     * to the signature.
     */
    // lifetime is needed so returned recipe does not outlive cookbook
    pub fn choose_recipe<'a>(&self, cookbook: & 'a Cookbook) -> Option<&'a Recipe> {
        //unimplemented!();
        for rec in cookbook.recipes() {
            if rec.diet() == self.diet() {
                return Some(rec);
            } 
        }
        None
    }
}
