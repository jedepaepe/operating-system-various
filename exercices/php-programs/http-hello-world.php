
<?php
// php -m | findstr socket
// php http-hello-world.php

// 1. Création du socket serveur
$serveur = socket_create(AF_INET, SOCK_STREAM, SOL_TCP);

if ($serveur === false) {
    die("Erreur socket : " . socket_strerror(socket_last_error()));
}

// 2. Association du socket au port 8080
socket_bind($serveur, "0.0.0.0", 8080);

// 3. Mise en écoute du serveur
socket_listen($serveur, 5);

echo "Serveur web démarré sur le port 8080\n";

// 4. Attente des connexions des clients
while (true) {

    // 5. Acceptation d'une connexion
    $client = socket_accept($serveur);

    echo "Un client est connecté.\n";

    // 6. Lecture de la requête HTTP
    $requete = socket_read($client, 1024);

    // 7. Préparation de la réponse HTTP
    $reponse =
        "HTTP/1.1 200 OK\r\n" .
        "Content-Type: text/html; charset=UTF-8\r\n" .
        "Connection: close\r\n" .
        "\r\n" .
        "<html>" .
        "<body>" .
        "<h1>Hello World !</h1>" .
        "<p>Mon premier serveur web en PHP.</p>" .
        "</body>" .
        "</html>";

    // 8. Envoi de la réponse au navigateur
    socket_write($client, $reponse, strlen($reponse));

    // 9. Fermeture de la connexion client
    socket_close($client);
}

// Fermeture du serveur
socket_close($serveur);

?>
