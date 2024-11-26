use std::{io::{self, Write}, mem::replace};

fn main() {
    let mut input_user = String::new();
    print!("Veuillez entrer une phrase : ");
    io::stdout().flush().unwrap();
    
    match io::stdin().read_line(&mut input_user) {
        Ok(_) => {
            let input_clean = input_user.trim();

            // Application des remplacements sur la chaîne collectée
            let change_caractere = input_clean
                .replace('a', "@")
                .replace("and", "&")
                .replace("et", "&")
                .replace('i', "1")
                .replace("de", "2");
            
            // Création d'un vecteur pour stocker les caractères à conserver
            let mut first_letters = Vec::new();
            let mut is_new_word = true;
            
            // Parcours de chaque caractère
            for c in change_caractere.chars() {
                // Si c'est un espace, le prochain caractère sera le début d'un nouveau mot
                if c.is_whitespace() {
                    is_new_word = true;
                    continue;
                }
                
                // Conserver les caractères de ponctuation spécifiés
                if c == '.' || c == ',' || c == '\'' || c == '!' || c == '?' {
                    first_letters.push(c);
                    continue;
                }
                
                // Pour les autres caractères, ne garder que la première lettre de chaque mot
                if is_new_word {
                    first_letters.push(c);
                    is_new_word = false;
                }
            }
            
            // Conversion du vecteur en String
            let result: String = first_letters.into_iter().collect();
            
            

            println!("Mot de passe généré : {}", result);
        }
        Err(error) => {
            eprintln!("Erreur lors de la lecture : {}", error);
        }
    }
}