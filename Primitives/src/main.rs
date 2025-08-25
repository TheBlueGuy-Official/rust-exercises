fn main() {
    // There 3 ways to assign type to a variable.
    let manual :i64 = 100;
    let auto = 10;
    let suffix = 50i8;
    // The last one was really new to me.

    // Also there are mutable and immutable variables. You can change the mutable
    let mut b = 256;

    println!("{}",b);

    b = 123454562i32;

    println!("{}",b);

    // I cannot change the type of the variable without shadowing.
    // b = true | This will cause an error.

    let b = true;

    // This is how I can set up an array.
    let my_array: [u8; 4] = [1, 2, 3, 4];

    // I cannot set an array with multiple types like string and bool but it's possible in tuple.
    let my_tuple = (10, 11u8, true, 'S', false);
}
