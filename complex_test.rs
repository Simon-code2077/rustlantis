// More complex test to trigger more compiler code paths
use std::collections::HashMap;

fn main() {
    // Test various Rust features to trigger more compiler code
    let mut map = HashMap::new();
    map.insert("hello", 42);
    
    // Pattern matching
    match map.get("hello") {
        Some(val) => println!("Found: {}", val),
        None => println!("Not found"),
    }
    
    // Closures
    let nums = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);
    
    // Generic function
    generic_function(42);
    generic_function("hello");
    
    // Trait usage
    let s = MyStruct { value: 100 };
    s.my_method();
}

fn generic_function<T: std::fmt::Debug>(x: T) {
    println!("Generic: {:?}", x);
}

struct MyStruct {
    value: i32,
}

trait MyTrait {
    fn my_method(&self);
}

impl MyTrait for MyStruct {
    fn my_method(&self) {
        println!("MyStruct value: {}", self.value);
    }
}