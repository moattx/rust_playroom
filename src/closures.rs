pub fn call_closure<F>(f: F) -> i32
where

    // FnOnce applies to closures that can be called once. All closures implement at least this trait, 
    // because all closures can be called. A closure that moves captured values out of its body will only implement FnOnce and 
    // none of the other Fn traits, because it can only be called once.
    //
    // FnMut applies to closures that don’t move captured values out of their body, but that might mutate the captured values. 
    // These closures can be called more than once.
    //
    // Fn applies to closures that don’t move captured values out of their body and that don’t mutate captured values, 
    // as well as closures that capture nothing from their environment. These closures can be called more than 
    // once without mutating their environment, which is important in cases such as calling a 
    // closure multiple times concurrently.
    

    // Fn: the closure captures by reference (&T). They are used for functions that can still be called if they only have reference 
    // access (with &) to their environment.
    //
    // FnMut: the closure captures by mutable reference (&mut T). They are used for functions that can be called if they have 
    // mutable reference access (with &mut) to their environment.
    //
    // FnOnce: the closure captures by value (T). They are used for functions that are 
    // only called once.

    F: FnOnce() -> i32,
{
    f()
}

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

    println!(
        "closure call from function: {}",
        call_closure(|| add_one_v4(5)),
    );

        let num_vec = vec![10, 9, 8];

    num_vec
        .iter()      // iterate over num_vec
        .enumerate() // get (index, number)
        .for_each(|(index, number)| println!("Index number {} has number {}", index, number)); // do something for each one

    Ok(())
}
