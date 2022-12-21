// a util that helps you look for files or directories
// ARGS -l list numbers of files and directories

use std::env;
use std::fs;

pub fn run() {
    // iterators produce a series of values, and we can call the collect method on an iterator to turn it into a collection, such as a vector
    let args: Vec<String> = env::args().collect();
    //dbg!(args);

    if args.len() > 1 && args[1] == "-l" {
        list_files();
    } else if args.len() > 1 {
        find_file(&args[1]);
    } else {
        println!("ERROR: no args");
    }
}

fn list_files() {
    let paths = fs::read_dir(".").unwrap(); // NO UNWRAP
    let mut num = 0;
    for _ in paths {
        num += 1;
    }
    println!("{}", num);
}

fn find_file(name: &str) {
    println!("name = {}", name);
    let paths = fs::read_dir(".").unwrap(); // NO UNWRAP
    for path in paths {
        if path.unwrap().path().display() == name{
            println!("{}", path.unwrap().path().display());
        }
    }
}

/*
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
*/
