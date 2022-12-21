// FUNCTIONS

// Function names are always in snake_case.

//Hint: if you define a function, the data it accepts are called parameters. If you call that function and pass data to it, 
//then it's called arguments.


// FUNCTIONS RETURN TUPLES


// public function for fns module. returns nothing 
// If no return type is specified for a function, it returns an empty tuple, 
// also known as a unit; an empty tuple is represented by ().
pub fn run() -> () {
    println!("this is a function :D");
    //secret();
    println!("secret return tuple = {:?}", secret_returns());

    // destructure the tuple into two variables names
    let (a, b) = secret_returns();
    println!("example of return tuple destructure: {}, {}", a, b);

    println!("the return type of a function that returns nothing is {:?}", do_nothing());

    // you dont need to use "return" in functions
    // return a unit. a unit is a empty tuple
    ()
}

// this makes it so the compiler stops complaing about unused funcitons
#[allow(dead_code)]

// this is also a function which is static by default because rust
fn secret(){
    println!("secret: only code in this file can see me");
}

// functions can return multiple values because functions return a tuple
fn secret_returns() -> (i32, i32) {
    (1,10)
}



// the return type is implied as ()
fn do_nothing() {
    // this function will return () if nothing is specified to return
}

