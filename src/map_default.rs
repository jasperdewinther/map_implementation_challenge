use crate::map::Map;
use std::collections::HashMap;

pub struct DefaultMap{
    data: HashMap<String, i32>
}

impl Map for DefaultMap{
    fn new() -> DefaultMap {
        DefaultMap {
            data: HashMap::new()
        }
    }


    fn insert(&mut self, key: &str, value: i32) -> bool {
        if self.data.contains_key(key){
            return false;
        }
        self.data.insert(key.to_string(), value);
        return true;
    }


    fn get(&mut self, key: &str) -> Option<&mut i32> {
        return self.data.get_mut(key);
    }

    fn remove(&mut self, key: &str) -> bool {
        match self.data.remove(key){
            Some(_) => return true,
            None => return false
        }
    }
}