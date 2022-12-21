//mod print;
//mod vars;
//mod types;
//mod fns;
//mod game;
//mod loops;
//mod ifnmatch;
//mod structs;
//mod genericsnerr;
//mod ownership;
//mod lifetimes;
//mod oop;
//mod lookfor;
//mod text;
mod closures;

// Be a good rustacean and properly use match when you can!
// USE MATCH MORE THAN IF STATEMENTS

// Main returns no value, but could return an error!
fn main() -> Result<(), String> {
    //print::run();
    //vars::run();
    //types::run();
    //fns::run();
    //loops::run();
    //game::run();
    //ifnmatch::run();
    //structs::run();
    //genericsnerr::run();
    //ownership::run();
    //lifetimes::run();
    //oop::run();
    //lookfor::run();
    //text::run();
    closures::run()?;

    // Unlike functions, methods are functions associated with a specific data type.
    //
    // static methods — methods that belong to a type itself are called using the :: operator.
    // Using a static method to create an instance of String
    //let s = String::from("Hello world!");

    // instance methods — methods that belong to an instance of a type are called using the . operator.
    // Using a method on the instance
    //println!("{} is {} characters long.", s, s.len());

    /*
    let result = do_something_that_might_fail(12);

    match result {
        Ok(v) => println!("found {}", v),
        Err(_e) => {
            // handle this error gracefully

            // return a new error from main that said what happened!
            return Err(String::from("something went wrong in main!"));
        }
    }
    */

    //Result is so common that Rust has a powerful operator ? for working with them.
    //These two statements are equivalent:
    //do_something_that_might_fail()?

    //match do_something_that_might_fail() {
    //    Ok(v) => v,
    //   Err(e) => return Err(e),
    //}

    // Look at how much code we saved!
    // let v = do_something_that_might_fail(22)?;
    // println!("found {}", v);

    // Notice we use a unit value inside a Result Ok
    // to represent everything is fine
    Ok(())
}

fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("this is not the right number"))
    }
}
