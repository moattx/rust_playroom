use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

// rust is sooo beautiful

pub fn run() {
    let mut random = rand::thread_rng().gen_range(1..=100);

    println!("lets play a game");
    println!("the game we'll play is a guessing a game where you guess the number!");

    loop {
        print!("enter: ");
        io::stdout().flush().unwrap();
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("didnt recieve input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("not a number or empty input! try again!");
                continue;
            }
        };
        print!("you entered: {}. ", guess);

        match guess.cmp(&random) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // update random number
                random = rand::thread_rng().gen_range(1..=100);
                println!("lets play again! (to exit please do ctrl + c)");
                continue;
            }
        }
    }
}
