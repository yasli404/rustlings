// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    call_me(3);
}

// Définition de la fonction call_me() qui prend un paramètre num de type u32
fn call_me(num: u32) {
    // Boucle for qui s'exécute num fois
    for i in 0..num {
        // Affiche "Ring! Call number x" où x est l'itération actuelle + 1
        println!("Ring! Call number {}", i + 1);
    }
}