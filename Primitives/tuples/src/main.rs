use std::fmt;

// I see. We can use tuple in a function.
fn reverce(tup: (bool, i32)) -> (i32, bool) {
    let (bool_rev, int_rev) = tup;
    (int_rev, bool_rev)
    // I don't understand why there is no ";" at the end of the last line. Let me study this ...
    // Ohhhh I see. now I know the differences between return (bool_param, int_param); and (bool_param, int_param).
}

fn main() {
    let my_tuple = (
        5u8, 6u16, 8u32, 10u64, 12i8, 20i16, -5i32, -100i64, true, "Hi", 's',
    );
    // It's possible to print a single part of the tuple.
    println!("This is the first index of the tuple: {}", my_tuple.0);
    println!("This is the second index of the tuple: {}", my_tuple.1);

    // We can use tuple in tuple line matrix in arrays.
    let tuple_in_tuple = (1, 2, 3, 4, 5, (6, 7, 8, 9, (10, 11, 12, 13)));

    println!("Using Debug Print to display tuple: {:?}", tuple_in_tuple);
    println!("Using Pretty Print to display tuple: {:#?}", tuple_in_tuple);
    // Long Tuples (more than 12 elements) cannot be printed!

    let tup = (true, 10i32);
    // Printing the reverce function.
    println!(
        "The tuple: {:?} | The reverse version: {:?}",
        tup,
        reverce(tup)
    );

    // Implementing fmt for a struct
    struct Matrix(f32, f32, f32, f32);

    impl fmt::Display for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Each row is printed on a new line, with parentheses around it
            write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
        }
    }
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);
}
