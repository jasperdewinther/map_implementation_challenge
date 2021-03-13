mod map_with_mat;
mod map_flat_vec;
mod map;
mod map_default;

use std::time::Instant;
use crate::map_flat_vec::FlatMap;
use crate::map_with_mat::MatMap;
use crate::map_default::DefaultMap;


// =====
// tests
// =====

fn check(condition: bool) { if !condition { panic!(); } }

fn run_test_suite<T: map::Map>() -> (f64, f64, f64, f64){

    let timer = Instant::now();
    let mut test1 = T::new();
    check(test1.get("random").is_none());
    let test_init = timer.elapsed().as_secs_f64();



    let timer = Instant::now();
    // test 1: very basic stuff
    let mut test1 = T::new();
    check(test1.get("random").is_none());
    test1.insert("random", 42);
    let random = test1.get("random");
    check(random.is_some());
    check(*random.unwrap() == 42);
    let test_1 = timer.elapsed().as_secs_f64();

    let timer = Instant::now();
    // test 2: add, remove and insert in different orders
    let mut test2 = T::new();
    check(test2.insert("hello", 1));
    check(test2.insert("hi", 2));
    check(!test2.insert("hello", 3));
    check(test2.insert("bye", 6));
    check(test2.remove("bye"));
    let value = test2.get("hello").unwrap();
    check(*value == 1);
    *value = 4;
    let value = test2.get("hello");
    check(value.is_some() && *value.unwrap() == 4);
    check(test2.remove("hello"));
    check(!test2.remove("hello"));
    check(test2.insert("greetings", 5));
    let value = test2.get("hi");
    check(value.is_some() && *value.unwrap() == 2);
    check(test2.get("hello").is_none());
    check(test2.get("bye").is_none());
    let test_2 = timer.elapsed().as_secs_f64();

    let timer = Instant::now();
    // test 3: add lots of numbers, then remove the even ones
    let mut test3 = T::new();
    for i in 0 .. 10000 {
        check(test3.get(&i.to_string()).is_none());
        check(test3.insert(&i.to_string(), i));
        check(i == 0 || test3.get(&(i - 1).to_string()).is_some());
        check(test3.get(&i.to_string()).is_some());
        check(test3.get(&(i + 1).to_string()).is_none());
    }
    for i in 0 .. 10000 {
        let ptr = test3.get(&i.to_string()).unwrap();
        check(*ptr == i);
        *ptr *= 2;
    }
    for i in 0 .. 10000 {
        let ptr = test3.get(&i.to_string());
        check(ptr.is_some() && *ptr.unwrap() == 2 * i);
        if i % 2 == 0 {
            check(test3.remove(&i.to_string()));
        }
    }
    let mut i = 0_i32;
    while i < 9999 {
        let ptr = test3.get(&(i + 1).to_string()).unwrap();
        check(*ptr == 2 * (i + 1));
        let key = i.to_string();
        check(!test3.remove(&key));
        check(test3.get(&key).is_none());
        i += 2;
    }
    let test_3 = timer.elapsed().as_secs_f64();

    return (test_init, test_1, test_2, test_3);
}

fn timing_wrapper<T: map::Map>(name: String){
    let mut suminit = 0.0;
    let mut sum1 = 0.0;
    let mut sum2 = 0.0;
    let mut sum3 = 0.0;
    for _ in 0..10{
        let results = run_test_suite::<T>();
        suminit +=results.0;
        sum1 +=results.1;
        sum2 +=results.2;
        sum3 +=results.3;
    }
    println!("{}", name);
    println!("init: {}", suminit/10.0);
    println!("1: {}", sum1/10.0);
    println!("2: {}", sum2/10.0);
    println!("3: {}", sum3/10.0);
}

fn main() {

    timing_wrapper::<FlatMap>("flatmap".parse().unwrap());
    timing_wrapper::<MatMap>("MatMap".parse().unwrap());
    timing_wrapper::<DefaultMap>("DefaultMap".parse().unwrap());

}
