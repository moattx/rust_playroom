// STRUCTS

// STRUCTS HAVE TO BEGIN WITH A CAPITAL LETTER
// no need for _?
struct SeaCreature {
    // String is a struct? i thought it was a vector whats going on?
    // vecs are structs?
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

pub fn run(){
    let ferris = SeaCreature{animal_type: "crab".to_string(), name: "ferris".to_string(),arms: 4,legs: 2,weapon: "cuteness".to_string()};
    println!(
        "{} is a {}. They have {} arms, {} legs, and a deadly weapon called {}",
        ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon
    );   
}
