pub fn run() -> Result<(), String> {
    // examples of closure syntax in variables
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    let add_one_v4 = |x| x + 1;

    println!(
        "{}{}{}{}",
        add_one_v1(1),
        add_one_v2(2),
        add_one_v3(3),
        add_one_v4(4)
    );
    // you cant attempt to call a closure whose types are inferred with two different types
    // ERROR:
    /*
        let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5);
    */

    //Closures can capture values from their environment in three ways,
    //which directly map to the three ways a function can take a parameter: borrowing immutably, borrowing mutably,
    //and taking ownership. The closure will decide which of these to use based on
    //what the body of the function does with the captured values.
    
    Ok(())
}
