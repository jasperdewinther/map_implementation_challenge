use crate::map::Map;
use bytemuck::from_bytes;

fn hash(data: &[u8]) -> u64{
    //http://www.cse.yorku.ca/~oz/hash.html
    let mut h: u64 = 5381;
    let c = data.chunks(4);
    c.for_each(|chunk| {
        let number: &u32 = from_bytes(chunk);
        h = ((h << 5)+h)+*number as u64;
    });
    return h;
}

pub struct FlatMap{
    data: Vec<(i32, Box<[u8]>)>
}

impl Map for FlatMap{
    fn new() -> FlatMap {
        FlatMap {
            data: Vec::new()
        }
    }

    fn insert(&mut self, key: &str, value: i32) -> bool {
        let found_index = self.data.binary_search_by(|(_, k)| {
            k.as_ref().cmp(key.as_bytes())
        });
        return match found_index {
            Ok(_) => { false }
            Err(i) => {
                self.data.insert(i, (value, Box::from(key.as_bytes())));
                true
            }
        }
    }

    fn get(&mut self, key: &str) -> Option<&mut i32> {
        let found_index = self.data.binary_search_by(|(_, k)| {
            k.as_ref().cmp(key.as_bytes())
        });
        match found_index{
            Ok(i) => {Some(&mut self.data.get_mut(i).unwrap().0)}
            Err(_) => {None}
        }
    }

    fn remove(&mut self, key: &str) -> bool {
        let found_index = self.data.binary_search_by(|(_, k)| {
            k.as_ref().cmp(key.as_bytes())
        });
        return match found_index {
            Ok(i) => {
                self.data.remove(i);
                true
            }
            Err(_) => {
                false
            }
        }
    }
}