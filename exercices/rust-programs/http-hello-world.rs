//
// serveur web simple qui retourne toujours une page HTML "hello world"
// pour compiler : rustc http-hello-world.rs
// pour exécuter : ./http-hello-world
//

use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {

    // 1. Création du socket serveur et écoute sur le port 8080
    let serveur = TcpListener::bind("0.0.0.0:8080")
        .expect("Impossible de démarrer le serveur");

    println!("Serveur web démarré sur le port 8080");

    // 2. Attente des connexions des clients
    for connexion in serveur.incoming() {

        // 3. Acceptation d'une connexion
        let mut client = connexion.expect("Erreur de connexion");

        println!("Un client est connecté.");

        // 4. Lecture de la requête HTTP
        let mut requete = [0; 1024];

        let _ = client.read(&mut requete);

        // 5. Préparation de la réponse HTTP
        let reponse =
            "HTTP/1.1 200 OK\r\n\
             Content-Type: text/html; charset=UTF-8\r\n\
             Connection: close\r\n\
             \r\n\
             <html>\
             <body>\
             <h1>Hello World !</h1>\
             <p>Mon premier serveur web en Rust.</p>\
             </body>\
             </html>";

        // 6. Envoi de la réponse au navigateur
        client.write_all(reponse.as_bytes())
            .expect("Erreur lors de l'envoi");

        // 7. Fermeture de la connexion client
        // La connexion est fermée automatiquement
        // lorsque la variable client sort de sa portée.
    }
}
