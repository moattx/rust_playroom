// OOP features like encapsulation, abstraction, and polymorphism
// there is no inheritance


// * The pub keyword exposes struct fields and methods outside of the module. *


struct SeaCreature {
    noise: String,
}

// implement METHOD SeaCreature of SeaCreature struct
// self is the struct
// get_sound is a function associated with the struct
impl SeaCreature { // this is a METHOD
    fn get_sound(&self) -> &str {
        &self.noise
    }
}

pub fn run() {
    let creature = SeaCreature {
        noise: String::from("blub"),
    };
    println!("{}", creature.get_sound());
}

