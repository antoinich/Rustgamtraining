use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("devinez le nombre!");

    let nombre_secret = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Veuillez entrer un nombre");

        let mut supposition = String::new();

        io::stdin().read_line(&mut supposition)
            .expect("Echec de la lecture de l'entrée utilisateur");
    
        let supposition: u32 = match supposition.trim().parse() {
            Ok(nombre) => nombre,
            Err(_) => continue,
        };
            
        println!("Votre nombre : {}", supposition);

        match supposition.cmp(&nombre_secret) {
            Ordering::Less => println!("c'est plus!"),
            Ordering::Greater => println!("C'est moins!"),
            Ordering::Equal =>  {
                println!("Bravo! Vous avez deviné");
                break;
        
            }
        }
    }
}
