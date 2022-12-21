/* this works?*/

/* formatting examples */

// rust makes functions static by default? and forces you to make it public
pub fn run() {
    // print to console
    println!("hello from the other side: print.rs file");

    // Basic formatting
    println!("{} is {} years old", "yusaf", 1);

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "yusaf", "Medinah", "code"
    ); /* 0 is yusaf. THE INDEX STARTS AT 0 */

    // Named Arguments
    println!(
        "{name} likes to play {game} and {name} likes to eat {food}",
        name = "yusaf",
        game = "football",
        food = "rice"
    );

    // Placeholder traits
    println!("binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);
    println!("binary: {:b}, Hex: {:x}, Octal: {:o}", 2, 2, 2);

    // Placeholder for debug trait
    println!("debug = {:?}", (12, true, "hello")); /* (12, true, "hello") is a tuple */

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
