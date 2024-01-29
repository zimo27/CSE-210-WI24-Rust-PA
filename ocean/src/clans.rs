use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct ClanSystem {
    // TODO: add necessary fields
    clans: HashMap<String, Vec<String>>,
}

impl ClanSystem {
    pub fn new() -> ClanSystem {
        //unimplemented!();
        ClanSystem { 
            clans: HashMap::new(), 
        }
    }

    /**
     * Returns a list of the names of the clan members for the given clan id.
     */
    pub fn get_clan_member_names(&self, clan_id: &str) -> Vec<String> {
        if self.clans.contains_key(clan_id) {
            return self.clans.get(clan_id).unwrap().to_vec();
        }
        return Vec::new()
        
    }

    /**
     * Returns the number of clans currently in existence.
     */
    pub fn get_clan_count(&self) -> usize {
        //unimplemented!();
        self.clans.len()
    }

    /**
     * Returns the number of clan members for the given clan id.
     */
    pub fn get_clan_member_count(&self, clan_id: &str) -> usize {
        self.clans.get(clan_id).map_or(0, |v| v.len())
    }

    /**
     * Returns the id of the clan with the most number of members, or None if such a clan does not exist.
     */
    pub fn get_largest_clan_id(&self) -> Option<String> {
        if let Some(res) = self.clans.iter().max_by_key(|x| x.1.len()) {
            Some(res.0.clone())
        } else {
            None
        }

    }

    /**
     * Add a new crab to a target clan
     */
    pub fn add_clan_member(& mut self, clan_id: &str, crab_name: &str) {
        if let Some(vector) = self.clans.get_mut(clan_id) {
            // If clan_id exists, append crab_name to the existing vector
            // avoid duplicate name
            if !vector.contains(&crab_name.to_string()) {
                vector.push(crab_name.to_string());
            }
            
        } else {
            // If clan_id does not exist, create a new vector and insert it
            let mut vector: Vec<String> = Vec::new();
            vector.push(crab_name.to_string());
            self.clans.insert(clan_id.to_string(), vector);
        }
    }
}