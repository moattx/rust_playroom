// LOOPS


pub fn run(){
    let mut x = 0;

    // infinite loop
    loop {
        x += 1;
        if x == 5 {
            break;
        }
    }

    println!("infinite loop: x is {}", x);
    
    x = 0;
    
    //loop can break to return a value.
        let v = loop {
        x += 1;
        if x == 13 {
            break "found the 13";
        }
    };
    println!("from loop: {}", v);

    x = 0;

    // while loop
    while x != 5 {
        x += 1;
        
        // cant do ++x or ++x in rust
        //++x;
    }
    println!("while loop: x is {}", x);

    
    // for item in iterator
    //
    // The .. operator creates an iterator that generates numbers from a start number up to but not including an end number.
    // The ..= operator creates an iterator that generates numbers from a start number up to and including an end number.
    for x in 0..5 {
        println!("{}", x);
    }

    for x in 0..=5 {
        println!("{}", x);
    }

    // prints infinetly from 0 to infinite
    for x in 0..{

        println!("{}", x);
    }
    
    
    


}





