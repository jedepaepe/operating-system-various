#
# ruby hello_server.rb
#
require 'socket'

# 1. Création du socket serveur
serveur = TCPServer.new('0.0.0.0', 8080)

puts "Serveur web démarré sur le port 8080"

# 2. Attente des connexions des clients
loop do

    # 3. Acceptation d'une connexion
    client = serveur.accept

    puts "Un client est connecté."

    # 4. Lecture de la requête HTTP
    requete = client.gets

    # 5. Préparation de la réponse HTTP
    contenu = <<~HTML
        <html>
            <body>
                <h1>Hello World !</h1>
                <p>Mon premier serveur web en Ruby.</p>
            </body>
        </html>
    HTML

    reponse =
        "HTTP/1.1 200 OK\r\n" \
        "Content-Type: text/html; charset=UTF-8\r\n" \
        "Content-Length: #{contenu.bytesize}\r\n" \
        "Connection: close\r\n" \
        "\r\n" \
        "#{contenu}"

    # 6. Envoi de la réponse au navigateur
    client.write(reponse)

    # 7. Fermeture de la connexion client
    client.close
end
