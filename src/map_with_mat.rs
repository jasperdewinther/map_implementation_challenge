use crate::map::Map;

const NUMBER_OF_CHARACTERS: usize = 126-32;
const NDARRAYSIZE: usize = NUMBER_OF_CHARACTERS.pow(2);


fn hash(data: &[u8]) -> u64{
    //http://www.cse.yorku.ca/~oz/hash.html
    let mut h: u64 = 5381;
    for point in data{
        h = ((h << 5)+h)+*point as u64;
    }
    return h;
}
#[inline]
fn index(x: &u8, y: &u8) -> usize{
    ((x-32) as usize)+((y-32) as usize)*NUMBER_OF_CHARACTERS
}

pub struct MatMap{
    data: Vec<Option<Vec<(i32, Box<[u8]>)>>>
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
                let i = index(&bytes[0], &32);
                let rest = &bytes[1..];
                f(self, i, rest)
            }
            0 => {
                let i = index(&32, &32);
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
        let vec:Option<&mut Vec<(i32, Box<[u8]>)>> = self.data[index].as_mut();
        return match vec {
            None => {
                let mut v: Vec<(i32, Box<[u8]>)> = Vec::new();
                v.push((value, Box::from(rest)));
                self.data[index] = Some(v);
                true
            }
            Some(mut v) => {
                let found_index = v.binary_search_by(|(_, key)| {
                    key.as_ref().cmp(rest)
                });
                match found_index {
                    Ok(_) => {
                        false
                    }
                    Err(i) => {
                        v.insert(i, (value, Box::from(rest)));
                        true
                    }
                }
            }
        }
    }
    fn handle_get_data(&mut self, index: usize, rest: &[u8]) -> Option<&mut i32>{
        match &mut self.data[index]{
            None => {return None}
            Some(v) => {
                let found_key = v.binary_search_by(|(_, key)| {
                    key.as_ref().cmp(rest)
                });
                match found_key{
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
            None => { false }
            Some(v) => {
                let found_key = v.binary_search_by(|(_, key)| {
                    key.as_ref().cmp(rest)
                });
                match found_key {
                    Ok(i) => {
                        v.remove(i);
                        true
                    }
                    Err(_) => {
                        false
                    }
                }
            }
        }
    }

}