fn main() {
    // As far I understood {} will be always a placeholder for the variable that is targeted to print.
    println!("Today is the {} day of learning rust.", 2);

    // Also there is something called positional arguments. You can position the placement of a variable in the print.
    println!(
        "{0} is the number of days in a year and I would love to be a junior rust developer in {0} days. Let's Gooo{1}oo",
        365, 0
    );

    // It's possible to name arguments
    println!(
        "{start} {mid} {ending}",
        start = "This is",
        mid = "sooo",
        ending = "fun and new to me."
    );

    // There is a way to change the formant of a number. This is beyond useful for the university homeworks :D
    println!("Base 10: {}", 43435);
    println!("Base 2: {:b}", 43435);
    println!("Base 8: {:o}", 43435);
    println!("Base 16: {:x}", 16);

    // I do not understand the text below, but I'm working on it.
    println!("{number:>5}", number = 5);
    // I see... It added 4 spaces and then printed 5. Let's test this more.
    println!("{number:=>2}", number = 5);
    // Interesting ... It didn't work the way that I wanted. It just added the = in the output.

    // There is a way to pad numbers. I have no idea why they call it padding numbers.
    println!("{number:0>5}", number = 5);
    // Let me do something ...
    println!("{number:W>5}", number = 5);
    println!("{number:⭐>5}", number = 5);
    //println!("{number:Blue<5}", number = 10); <I can't add multiple characters>
    // Brother cooked ...

    // Apparently by changing the > I can change the side of padding.
    println!("{number:B<5}", number = 5);

    // I can use a variable instead of giving the exact number to use spaces.
    println!("{number:>count$}", number = 5, count = 5);
}
