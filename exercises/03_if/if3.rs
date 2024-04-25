// if3.rs
//
// Execute `rustlings hint if3` or use the `hint` watch subcommand for a hint.

pub fn animal_habitat(animal: &str) -> &'static str {
    let identifier = if animal == "crab" {
        // Remplacé par "crab" pour identifier correctement l'animal
        "crab"
    } else if animal == "gopher" {
        // Remplacé par "gopher" pour identifier correctement l'animal
        "gopher"
    } else if animal == "snake" {
        // Remplacé par "snake" pour identifier correctement l'animal
        "snake"
    } else {
        // Laissé "Unknown" pour les animaux inconnus
        "Unknown"
    };

    // DO NOT CHANGE THIS STATEMENT BELOW
    let habitat = if identifier == "crab" {
        "Beach"
    } else if identifier == "gopher" {
        "Burrow"
    } else if identifier == "snake" {
        "Desert"
    } else {
        "Unknown"
    };

    habitat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}