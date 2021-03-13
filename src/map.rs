



pub trait Map{
    fn new() -> Self;
    fn insert(&mut self, key: &str, value: i32) -> bool;
    fn get(&mut self, key: &str) -> Option<&mut i32>;
    fn remove(&mut self, key: &str) -> bool;
}