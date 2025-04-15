use std::io;
use rand :: Rng;

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
 


fn lettre() {
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


}   */


fn main() {
    println!("choisis un niveau de difficulé");
    println!("1. facile (1 à 10, essais illimités");
    println!("2. medium (1 à 50, 10 essais max");
    println!("3. difficile (1 à 100, max 7 essais)");

    let mut niveau = String::new();
    io::stdin().read_line(&mut niveau).expect("Erreur lors de la lecture");


    let (limite,max_essais) = match niveau.trim() {
        "1"=> (10,None),
        "2"=> (50, Some(10)),
        "3"=> (100, Some(7)),
        _ =>{
            println!("L'entrée n'est pas bonne, fin du programme");
        return;
    },
    };


    let secret = rand::thread_rng().gen_range(1..=limite);
    let mut compteur = 0;

    loop {
        let mut input = String::new();

        if let Some(max) = max_essais {
            if compteur >= max {
                println!("T'as perdu !");
                break;
            }
        }
        
        println!("Tape un chiffre");
        io::stdin()
            .read_line(&mut input)
            .expect("Erreur lors de la lecture");

        let guess: u32 = input.trim().parse().expect("pas un nombre");
        compteur += 1;
        if guess > secret {
            println!("Trop grand\n");
        } else if guess < secret {
            println!("Trop petit\n");
        } else {
            println!("C'est le bon !");
            break;

        }
        
        
        
    }
    println!("Tu as réussi en : {} coups", compteur)
}