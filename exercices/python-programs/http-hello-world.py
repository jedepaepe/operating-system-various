# serveur web simple qui retourne toujours une page HTML "hello world"
# pour exécuter : python http-hello-world.py

import socket

# 1. Création du socket serveur
serveur = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

# 2. Configuration de l'adresse et du port
serveur.bind(("0.0.0.0", 8080))

# 3. Mise en écoute du serveur
serveur.listen(5)

print("Serveur web démarré sur le port 8080")

# 4. Attente des connexions des clients
while True:

    # Acceptation d'une connexion
    client, adresse = serveur.accept()

    print("Connexion de :", adresse)

    # Lecture de la requête HTTP
    requete = client.recv(1024)

    # Préparation de la réponse HTTP
    reponse = (
        "HTTP/1.1 200 OK\r\n"
        "Content-Type: text/html; charset=UTF-8\r\n"
        "Connection: close\r\n"
        "\r\n"
        "<html>"
        "<body>"
        "<h1>Hello World !</h1>"
        "<p>Mon premier serveur web en Python.</p>"
        "</body>"
        "</html>"
    )

    # Envoi de la réponse
    client.sendall(reponse.encode("utf-8"))

    # Fermeture de la connexion client
    client.close()
