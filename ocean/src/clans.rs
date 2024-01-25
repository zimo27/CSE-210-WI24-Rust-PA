use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct ClanSystem {
    // TODO: add necessary fields
    clans:HashMap<String, Vec<String>>,
}

impl ClanSystem {
    pub fn new() -> ClanSystem {
        //unimplemented!();
        ClanSystem { 
            clans: (HashMap::new()), 
        }
    }

    /**
     * Returns a list of the names of the clan members for the given clan id.
     */
    pub fn get_clan_member_names(&self, clan_id: &str) -> Vec<String> {
        //unimplemented!();
        /*let mut exist:u8 = 0;
        for (key, value) in self.clans.iter() {
            if key.to_string() == clan_id {
                exist = 1;
                return value.to_vec();
            }
        }*/
        if self.clan_exist(clan_id) {
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
        //unimplemented!();
        //println!("{}",self.clans.get(clan_id).unwrap().len());
        if !self.clan_exist(clan_id) {
            return 0;
        }
        //self.clans.get(clan_id).unwrap().len()
        /* 
        let mut ans:usize = 0;
        for (key, value) in self.clans.iter() {
            if key.to_string() == clan_id {
                ans = value.to_vec().len();
            }
        }
        return ans;
        */
        return self.clans.get(clan_id).unwrap().to_vec().len();
    }

    /**
     * Returns the id of the clan with the most number of members, or None if such a clan does not exist.
     */
    pub fn get_largest_clan_id(&self) -> Option<String> {
        //unimplemented!();
        if self.get_clan_count() == 0 {
            return None;
        }
        let mut max_num:usize = 0;
        let mut clan_name:String = "".to_string();
        for (key, value) in self.clans.iter() {
            if value.to_vec().len() >= max_num {
                max_num = value.to_vec().len();
                clan_name = key.to_string();
            }
        }
        Some(clan_name)

    }

    pub fn clan_exist(&self, clan_id: &str) -> bool{
        let mut exist:u8 = 0;
        for (key, value) in self.clans.iter() {
            if key.to_string() == clan_id {
                exist = 1;
            }
        }
        return exist==1;
    }


    pub fn add_clan_member(& mut self, clan_id: &str, crab_name: &str) {
        /* 
        let mut exist:u8 = 0;
        for (key, value) in self.clans.iter() {
            if key.to_string() == clan_id {
                value.to_vec().push(crab_name.to_string());
                exist = 1;
            }
        }
        if exist==0 {
            let mut vector:Vec<String> = Vec::new();
            vector.push(crab_name.to_string());
            self.clans.insert(clan_id.to_string(), vector);
            //self.clans.get_mut(clan_id).unwrap().push(crab_name.to_string());
        }
        */

        /* 
        if self.clan_exist(clan_id) {
            self.clans.get_mut(clan_id).unwrap().push(crab_name.to_string());
        } else {
            let mut vecto:Vec<String> = Vec::new();
            vecto.push(crab_name.to_string());
            self.clans.insert(clan_id.to_string(), vecto);
        }*/
        if let Some(vector) = self.clans.get_mut(clan_id) {
            // If clan_id exists, append crab_name to the existing vector
            vector.push(crab_name.to_string());
        } else {
            // If clan_id does not exist, create a new vector and insert it
            let mut vector: Vec<String> = Vec::new();
            vector.push(crab_name.to_string());
            self.clans.insert(clan_id.to_string(), vector);
        }
    }
}