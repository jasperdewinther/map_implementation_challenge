use crate::map::Map;

const NUMBER_OF_CHARACTERS: usize = 126-31;
const NDARRAYSIZE: usize = NUMBER_OF_CHARACTERS.pow(2);


fn hash(data: &[u8]) -> u128{
    let mut h = 0u128;
    for point in data{
        h = (h << 8).wrapping_add(*point as u128);
    }
    return h;
}
#[inline]
fn index(x: &u8, y: &u8) -> usize{
    ((x-31) as usize)+((y-31) as usize)*NUMBER_OF_CHARACTERS
}

pub struct MatMap{
    data: Vec<Option<Vec<(i32, u128)>>>
}

impl Map for MatMap  {
    fn new() -> MatMap {
        let mut data = Vec::with_capacity(NDARRAYSIZE);
        for _ in 0..NDARRAYSIZE{
            data.push(None);
        }
        MatMap {
            data
        }
    }



    fn insert(&mut self, key: &str, value: i32) -> bool {
        return self.dim_matcher(key,
                                |map, i, rest|{
                                    map.handle_insert_data(i, rest, value)
                                }
        );
    }



    fn get(&mut self, key: &str) -> Option<&mut i32> {
        return self.dim_matcher(key,
                                |map, i, rest|{
                                    map.handle_get_data(i, rest)
                                }
        );
    }


    fn remove(&mut self, key: &str) -> bool {
        return self.dim_matcher(key,
                                |map, i, rest|{
                                    map.handle_remove(i, rest)
                                }
        );
    }
}

impl MatMap{
    fn dim_matcher<'a, T>(&'a mut self,key: &str, f: impl Fn(&'a mut Self, usize, &[u8]) -> T) -> T{
        let bytes = key.as_bytes();
        return match bytes.len() {
            1 => {
                let i = index(&bytes[0], &31);
                let rest = &bytes[1..];
                f(self, i, rest)
            }
            0 => {
                let i = index(&31, &31);
                let rest = bytes;
                f(self, i, rest)
            }
            _ => {
                let i = index(&bytes[0], &bytes[1]);
                let rest = &bytes[2..];
                f(self, i, rest)
            }
        }
    }

    fn handle_insert_data(&mut self, index: usize, rest: &[u8], value: i32) -> bool{
        let vec:Option<&mut Vec<(i32, u128)>> = self.data[index].as_mut();
        return match vec {
            Some(v) => {
                let key_hash = hash(rest.as_ref());
                let found_index = v.binary_search_by(|(_, k)| {
                    k.cmp(&key_hash)
                });
                match found_index {
                    Ok(_) => {
                        false
                    }
                    Err(i) => {
                        v.insert(i, (value, key_hash));
                        true
                    }
                }
            }
            _ => {
                let mut v: Vec<(i32, u128)> = Vec::new();
                let key_hash = hash(rest.as_ref());
                v.push((value, key_hash));
                self.data[index] = Some(v);
                true
            }
        }
    }
    fn handle_get_data(&mut self, index: usize, rest: &[u8]) -> Option<&mut i32>{
        match &mut self.data[index]{
            None => {return None}
            Some(v) => {
                let key_hash = hash(rest.as_ref());
                let found_index = v.binary_search_by(|(_, k)| {
                    k.cmp(&key_hash)
                });
                match found_index{
                    Ok(index) => {
                        let object = v.get_mut(index).unwrap();
                        Some(&mut object.0)
                    }
                    Err(_) => {
                        return None;
                    }
                }
            }
        }
    }
    fn handle_remove(&mut self, index: usize, rest: &[u8]) -> bool{
        return match &mut self.data[index] {
            Some(v) => {
                let key_hash = hash(rest.as_ref());
                let found_index = v.binary_search_by(|(_, k)| {
                    k.cmp(&key_hash)
                });
                match found_index {
                    Ok(i) => {
                        v.remove(i);
                        true
                    }
                    Err(_) => {
                        false
                    }
                }
            }
            _ => { false }
        }
    }

}