use std::io;


struct Contact {
    nom : String,
    prenom : String,
    email : String,
    telephone : String,
}


fn main() {
    println!("nom du nouveau contact");
    let mut nom_c = String::new();
    io::stdin()
        .read_line(&mut nom_c)
        .expect("erreur");
    nom_c = nom_c.trim().to_string();
    

    println!("prénom du contact");
    let mut prenom_c = String::new();
    io::stdin()
        .read_line(&mut prenom_c)
        .expect("erreur");
    prenom_c = prenom_c.trim().to_string();

    println!("email du contact");
    let mut email_c = String::new();
    io::stdin()
        .read_line(&mut email_c)
        .expect("Erreur");
    email_c = email_c.trim().to_string();
    
    println!("numéro du contact");
    let mut numero_c = String::new();
    io::stdin() 
        .read_line(&mut numero_c)
        .expect("Erreur");
    numero_c = numero_c.trim().to_string();
    
    println!(" nom: {},\n Prénom: {},\n Email: {},\n numero: {}", nom_c, prenom_c, email_c, numero_c)
}