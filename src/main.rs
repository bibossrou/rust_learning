use std::io;

// Fonction demande d'un prénom

/* 
fn prenom() {
    let mut entree = String::new();

    println!("tape ton nom");

    io::stdin()
        .read_line(&mut entree)
        .expect("Erreur");
    
    println!("tu as entré : {}", entree.trim());

}

// Fonction age mineur ou majeur

fn age() {
    let mut input = String::new();

    println!("tape ton age");

    io::stdin()
        .read_line(&mut input)
        .expect("Erreur lors de la lecture");

    let age : u32 = input.trim().parse().expect("Pas un nombre");
    
    if age < 18 {
        println!("tu es mineur");
    } else {
        println!("tu es majeur");
    }

    
}
    */


fn main() {
    let mut input = String::new();

    println!("Tape une lettre");

    io::stdin()
        .read_line(&mut input)
        .expect("Erreur lors de la lecture");

    let lettre = input.to_lowercase().trim().chars().next();
    
    match lettre {
        Some('a') | Some('e') | Some('i') | Some('o') | Some('u') | Some('y') => println!("Voyelle"),
        Some('b' ..='z') => println!("Consonne"),
        _ => println!("Ce n'est pas une lettre"),
    
    }


}