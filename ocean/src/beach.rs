use crate::color::Color;
use crate::crab::Crab;
use crate::diet::Diet;
use crate::clans::ClanSystem;
use std::slice::Iter;

#[derive(Debug)]
pub struct Beach {
    // TODO: Declare the fields of the Beach struct here.
    pub crabs: Vec<Crab>,
    // need a data structure to contain the crabs
    pub clans: ClanSystem,
}

impl Beach {
    pub fn new() -> Beach {
        //unimplemented!();

        // not sure if the declaration of crabs makes it mutable, I think not
        Beach { 
            crabs: (Vec::new()),
            clans: (ClanSystem::new()),
        }
    }

    /**
     * Returns the number of crabs on the beach.
     */
    pub fn size(&self) -> usize {
        self.crabs.len()
    }

    /**
     * This moves `crab`, taking ownership. Do NOT implement Copy for Crab.
     *
     *   - After `add_crab` returns:
     *     - The Beach should hold the crab in its collection of crabs.
     *     - The newly added crab should be at the END of the collection.
     */
    pub fn add_crab(&mut self, crab: Crab) {
        self.crabs.push(crab);
    }

    pub fn get_crab(&self, index: usize) -> &Crab {
        &self.crabs[index]
    }

    pub fn crabs(&self) -> Iter<Crab> {
        self.crabs.iter()

    }

    /**
     * Returns:
     *   - None if the beach is empty.
     *   - Some of a reference to the Crab with the highest speed.
     */
    pub fn get_fastest_crab(&self) -> Option<&Crab> {
       // unimplemented!();
       if self.size()==0 {
            None
       } else {
            let mut max_speed: u32 = 0;
            let mut f_crab: &Crab = &self.crabs[0];
            for cra in self.crabs() {
                if cra.speed() > max_speed {
                    max_speed = cra.speed();
                    f_crab = cra

                }
            }
            Some(f_crab)

       }
    }

    /**
     * Returns a vector of references to the crabs with a given name.
     */
    pub fn find_crabs_by_name(&self, name: &str) -> Vec<&Crab> {
        //unimplemented!();
        let mut arr:Vec<&Crab> = Vec::new();
        for cra in self.crabs() {
            if cra.name() == name {
                arr.push(cra);
            }
        }
        arr
    }

    /**
     * Breeds the `Crab`s at indices `i` and `j`, adding the new `Crab` to
     * the end of the beach's crab vector. If the indices are out of bounds,
     * the method should panic.
     */
    pub fn breed_crabs(&mut self, i: usize, j: usize, name: String) {
        //unimplemented!();
        self.crabs.push(Crab::breed(self.get_crab(i), self.get_crab(j), name));
    }

    /**
     * Returns a reference to the clan system associated with the beach.
     */
    pub fn get_clan_system(&self) -> &ClanSystem {
        //unimplemented!();
        &self.clans
    }

    /**
     * Adds a crab that lives on the beach as a member to the clan system for the given clan id and the crab's name.
     * A crab can only belong to one clan.
     */
    pub fn add_member_to_clan(&mut self, clan_id: &str, crab_name: &str) {
        //unimplemented!();
        self.clans.add_clan_member(clan_id, crab_name);
    }

    /**
     * Returns the id of the clan that wins the competition given two clan ids. The winner is decided based on the average speed of the clan members.
     * Return `None` if there are no clear winners between two different existing clans. If the inputs are invalid, return an Err string.
     */
    pub fn get_winner_clan(&self, id1: &str, id2: &str) -> Result<Option<String>, String> {
        //unimplemented!();
        if self.clans.get_clan_member_names(id1).is_empty() || self.clans.get_clan_member_names(id2).is_empty()  {
            return Err("Invalid input for get_winner_clan".to_string());
        }

        // get clan menber names from clan methods, then get speed from self.crabs, then average them
         
        let mut count1:u32 = 0;
        let mut speedSum1:u32 = 0;
        let mut count2:u32 = 0;
        let mut speedSum2:u32 = 0;
        for crab in self.crabs.iter() {
            if self.clans.get_clan_member_names(id1).contains(&crab.name().to_string()) {
                count1+=1;
                speedSum1+=crab.speed();
            } else if self.clans.get_clan_member_names(id2).contains(&crab.name().to_string()) {
                count2+=1;
                speedSum2+=crab.speed();
            }
        }
        

        let mut speed1:u32 = 0;
        let mut speed2:u32 = 0;
        if count1>0 {
            speed1 = speedSum1/count1;
        }
        if count2>0 {
            speed2 = speedSum2/count2;
        }

        if  speed1 == speed2  {
            return  Ok(None);
        } else if speed1 > speed2 {
            return Ok(Some(id1.to_string()));
        } else {
            return Ok(Some(id2.to_string()));
        }
        
        

        
    }
}
