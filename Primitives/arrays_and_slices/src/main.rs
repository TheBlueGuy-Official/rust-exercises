use std::mem;

// Here $i32 is used to borrow the value not to copy it but the value it cannot be change because in it imutable.
// To make it imutable I should use mut (slice: &mut i32)
fn analyze_slice(slice: &[i32]) {
    println!("The first element of the slice is {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    // This is how I can let an array
    let my_array: [i8; 5] = [1, 2, 3, 4, 5];
    // This is the way to set an array with the same value
    let autofill: [i32; 250] = [0; 250];

    println!("The first element is : {}", my_array[0]);
    // println! ("The first element is : {}", my_array[-1]);
    println!("Number of elements in array is {}", my_array.len());

    // This the way to calculate how many bytes is occupied by the variable (INTERESTING...)
    println!("5 element but has numbers: {}", mem::size_of_val(&my_array));
    println!(
        "250 elements but has 0 as all of them: {}",
        mem::size_of_val(&autofill)
    );

    analyze_slice(&autofill);

    println!("Borrow just a part of an array.");
    analyze_slice(&autofill[1..5]);

    // Generic for
    for i in 0..my_array.len() {
        match my_array.get(i) {
            Some(j) => println!("Index {}: Value {}", i, j),
            None => println!("Index {} does not exist in array!", i),
        }
    }
}
