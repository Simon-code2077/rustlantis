fn process_array(arr: [i32; 4]) -> i32 {
    let mut sum = 0;
    for i in 0..4 {
        sum = sum.wrapping_add(arr[i]);
    }
    sum
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a.wrapping_add(b)
}

fn multiply_by_two(x: i32) -> i32 {
    x.wrapping_mul(2)
}

fn main() {
    println!("Running combined functions:");

    // Call process_array
    // process_array();

    // Call add_numbers
    // add_numbers();

    // Call multiply_by_two
    // multiply_by_two();

    println!("All functions completed.");
}
