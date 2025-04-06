use std::io;

// Fonction demande d'un prénom


fn prenom() {
    let mut entree = String::new();

    println!("tape ton nom");

    io::stdin()
        .read_line(&mut entree)
        .expect("Erreur");
    
    println!("tu as entré : {}", entree.trim());

}

// Fonction age mineur ou majeur

fn main() {
    let mut input = String::new();

    println!("tape ton age");

    io::stdin()
        .read_line(&mut input);

    let age : u32 = input.trim().parse().expect("Pas un nombre");
    
    if age < 18 {
        println!("tu es mineur");
    } else {
        println!("tu es majeur");
    }

    
}