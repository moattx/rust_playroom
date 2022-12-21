/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.

// usize depends on the arch if the arch is 32 bits then usize is 32 bits

/*
 * Should avoid using f32, unless you need to reduce memory consumption badly or if you are doing
 * low-level optimization, when targeted hardware does not support for double-precision or when
 * single-precision is faster than double-precision on it.
 */

//    If you are looking for a dynamic/ growable array, you can use vectors. Vectors can contain
//    any type of elements but all elements must be in the same data type.
//    Vec is a struct, but internally it contains a reference to a fixed list of its items on the heap.
//
//    A vector starts with a default capacity; when more items are added than it has capacity for, 
//    it reallocates its data on the heap to have a new fixed list with large capacity.

pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 4545445454545;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    // Get boolean from expression
    let is_greater: bool = 10 < 5;

    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));

    // Constans are just like macros; they get copied and pasted by the compiler
    const SCREAMING_SNAKE_CASE_F64: f64 = 2.2222;
    const SCREAMING_SNAKE_CASE_STR: &str = "THIS HAS BEEN COPIED AND PASTED BY THE COMPILER";
    println!(
        "constants are just like c macros: f64 constant = {} str constant = {} ",
        SCREAMING_SNAKE_CASE_F64, SCREAMING_SNAKE_CASE_STR
    );

    // An array is a fixed length collection of data elements all of the same type.
    // this means nothing could be added nor removed (it isnt stored in the heap)
    let nums: [i32; 3] = [1, 2, 3];
    println!("{:?}", nums);
    println!("{}", nums[1]);


    // We can be explicit with type
    let mut i32_vec = Vec::<i32>::new(); // turbofish <3
    i32_vec.push(1);
    i32_vec.push(2);
    i32_vec.push(3);

    // But look how clever Rust is about determining the type automatically
    let mut float_vec = Vec::new();
    float_vec.push(1.3);
    float_vec.push(2.3);
    float_vec.push(3.4);

    // That's a beautiful macro!
    let string_vec = vec![String::from("Hello"), String::from("World")];

    // Vec has the method iter() which creates an iterator from a vector, allowing us to easily put a vector into a for loop.
    for word in string_vec.iter() {
        println!("{}", word);
    }

}
