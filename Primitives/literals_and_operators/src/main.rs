fn main() {
    // Integers, floats, chars, strings, booleans and unit types are avaiable in rust.

    println!("1 + 2 = {}", 1u8 + 2);
    println!("1 - 2 = {}", 1i8 + 2);
    // If I change the i to u it will cause an error. Beacuse there is no - in u.

    // Rust also supports E-notaion.
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Boolean logic is accepted here.
    println!("true AND false = {}", true && false);
    println!("true OR false = {}", true || false);
    println!("NOT true = {}", !true);

    // Operation in bits is also allowed.
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    // I don't know why and where it becomes useful...

    // There is a really cool feature that I haven't seen before is using _ to improve readability.
    println!("This is how you can print one billion in number:{} ",1_000_000_000u32);
}
