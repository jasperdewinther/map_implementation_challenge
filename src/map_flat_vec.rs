use crate::map::Map;

#[inline]
fn hash(data: &[u8]) -> u128{
    //http://www.cse.yorku.ca/~oz/hash.html
    let mut h = 0u128;
    for point in data{
        h = (h << 8).wrapping_add(*point as u128);
    }
    return h;
}

pub struct FlatMap{
    data: Vec<(i32, u128)>
}

impl Map for FlatMap{
    fn new() -> FlatMap {
        FlatMap {
            data: Vec::new()
        }
    }

    fn insert(&mut self, key: &str, value: i32) -> bool {
        let key_hash = hash(key.as_ref());
        let found_index = self.data.binary_search_by(|(_, k)| {
            k.cmp(&key_hash)
        });
        return match found_index {
            Ok(_) => { false }
            Err(i) => {
                self.data.insert(i, (value, key_hash));
                true
            }
        }
    }

    fn get(&mut self, key: &str) -> Option<&mut i32> {
        let key_hash = hash(key.as_ref());
        let found_index = self.data.binary_search_by(|(_, k)| {
            k.cmp(&key_hash)
        });
        match found_index{
            Ok(i) => {Some(&mut self.data.get_mut(i).unwrap().0)}
            Err(_) => {None}
        }
    }

    fn remove(&mut self, key: &str) -> bool {
        let key_hash = hash(key.as_ref());
        let found_index = self.data.binary_search_by(|(_, k)| {
            k.cmp(&key_hash)
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