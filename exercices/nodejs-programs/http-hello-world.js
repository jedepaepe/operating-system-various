// serveur web simple qui retourne toujours une page HTML "hello world"
// pour exécuter : node http-hello-world.js

const http = require('http');

// Création du serveur web
const serveur = http.createServer((requete, reponse) => {

    // Préparation de la réponse HTTP
    reponse.writeHead(200, {
        'Content-Type': 'text/html; charset=UTF-8'
    });

    // Envoi de la réponse
    reponse.end(`
        <html>
            <body>
                <h1>Hello World !</h1>
                <p>Mon premier serveur web en Node.js.</p>
            </body>
        </html>
    `);
});

// Mise en écoute sur le port 8080
serveur.listen(8080, () => {
    console.log('Serveur démarré sur le port 8080');
});
