// String literals are always Unicode
// LEN gets the length of the string literal in bytes (not number of characters).
// LEN GETS THE LENGTH OF THE STRING LITERAL IN BYTES (NOT NUMBER OF CHARECTORS).
// let s = String::from("ラウトは難しいです！THIS IS CHINESE BTW
// s.len() // returns 30. WHICH IS WRONG
// TO GET THE ACTUAL NUMBER OF CHARS YOU DO s.chars().count()

pub fn run() {
    //    String Literals
    //
    //String literals are always Unicode.
    //
    //String literals type are &'static str:
    //
    //    & meaning that it's referring to a place in memory, and it lacks a
    //          &mut meaning that the compiler will not allow modification
    //    'static meaning the string data will be available till the end of our program (it never drops)
    //    str means that it points to a sequence of bytes that are always valid utf-8
    //
    //Memory details:
    //
    //    The Rust compiler will likely put your string in the data segment of your program memory

    let a: &'static str = "hi 🦀";
    println!("{} {}", a, a.len());

    // \0 = null
    let a: &'static str = "Ferris says:\0\"hello\"";
    println!("{}", a);
    //https://doc.rust-lang.org/reference/tokens.html

    // Multi-line String Literals
    // Rust strings are multiline by default
    let haiku = "
        I write, erase, rewrite
        Erase again, and then
        A poppy blooms.
        - Katsushika Hokusai";
    println!("{}", haiku);

    //Use a \ at the end of a line if you don't want newline
    println!(
        "hello \
    world"
    ); // notice that the spacing before w is ignored

    //Raw String Literals
    //Raw strings allow us to write a sequence of characters verbatim by starting
    //with r#" and ending with "#. It lets us insert characters that might otherwise confuse
    //a normal string as literals (like double quotes and backslashes).
    let a: &'static str = r#"
        <div class="advice">
            Raw strings are useful for some situations.
        </div>
        "#;
    println!("{}", a);

    //String Literals From Files
    // include_str! is a macro that gets text from files
    let capacity = include_str!("/sys/class/power_supply/BAT0/capacity");
    println!("{}", capacity);
    println!("{}", capacity.trim_end());
    let a = "hi test";
    println!("(BYTES) len = {} chars = {} ", a.len(), a.chars().count());
    let first_word = &a[0..2];
    let second_word = &a[3..];

    // let half_crab = &a[3..5]; FAILS
    // Rust does not accept slices of invalid unicode characters
    println!("{} {}", first_word, second_word);

    //Common methods of &str:

    println!("(BYTES) len = {} chars = {} ", a.len(), a.chars().count());
    //len gets the length of the string literal in bytes (not number of characters).
    //to get the number of charectors you do a.chars.count();

    if a.starts_with("hi") {
        println!("it starts with hi");
    } else {
        println!("doesnt start with hi");
    }
    if a.ends_with("test") {
        println!("it ends with test");
    } else {
        println!("it doesnt end with test");
    }
    // starts_with/ends_with for basic testing.
    // starts_with returns true if a starts with or ends with

    if a.is_empty() {
        println!("is empty");
    } else {
        println!("not empty");
    }
    //is_empty returns true if zero length.

    //find returns an Option<usize> of the first position of some text.

    if a.contains("est") {
        println!("it contians est");
    } else {
        println!("it does not contain est");
    }

    // CHARS
    // With so much difficulty in working with Unicode, Rust offers a way to retrieve a sequence of utf-8 bytes as a vector of characters of type char.

    // A char is always 4 bytes long (allowing for efficient lookup of individual characters).

    // collect the characters as a vector of char
    let chars = "hi O".chars().collect::<Vec<char>>();
    println!("{}", chars.len()); // should be 4
                                 // since chars are 4 bytes we can convert to u32
    println!("{}", chars[3] as u32);

    //String
    //
    //A String is a struct that owns a sequence of utf-8 bytes in heap memory.
    //
    //Because its memory is on the heap, it can be extended, modified, etc. in ways string literals cannot.
    //
    //Common methods:
    //
    //    push_str to add more utf-8 bytes to the end of a string.
    //    replace to replace sequences of utf-8 bytes with others.
    //    to_lowercase/to_uppercase for case changes.
    //    trim for trimming space
    //
    //When a String is dropped, its heap memory is also dropped.
    //
    //String has a + operator that extends the string with a &str and returns itself, but it might not be as ergonomic as you hope for.

    let mut helloworld = String::from("hello");
    helloworld.push_str(" world");
    helloworld = helloworld + "!";
    println!("{}", helloworld);

    //Text As Function Parameters
    //String literals and strings are generally passed around as a string slice to functions.
    //This offers a lot of flexibility for most scenarios where you don't actually have to pass ownership.
    // say_it_loud can borrow &'static str as a &str

    say_it_loud("hello");
    // say_it_loud can also borrow String as a &str
    say_it_loud(&String::from("goodbye"));

    // Building Strings
    // concat and join are two simple but powerful ways for building strings.
    let helloworld = ["hello", " ", "world", "!"].concat();
    let abc = ["a", "b", "c"].join(",");
    println!("{}", helloworld);
    println!("{}", abc);

    //Converting Strings
    //
    //Many types can be converted to a string using to_string.
    //
    //The generic function parse can be used to convert strings or string literals into a typed value.
    //This function returns a Result because it could fail.
    string_convert_example().unwrap();
}
fn say_it_loud(msg: &str) {
    println!("{}!!!", msg.to_string().to_uppercase());
}

fn string_convert_example() -> Result<(), std::num::ParseIntError> {
    let a = 42;
    let a_string = a.to_string();
    let b = a_string.parse::<i32>()?;
    println!("{} {}", a, b);
    Ok(())
}
