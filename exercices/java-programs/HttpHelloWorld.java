
// Java est un rien plus compliqué
// le nom de la classe doit être le même que le nom du fichier
// pour compiler : javac HttpHelloWorld.java
// pour exécuter : java HttpHelloWorld

import java.net.ServerSocket;
import java.net.Socket;
import java.io.OutputStream;

public class HttpHelloWorld {

    public static void main(String[] args) throws Exception {

        // 1. Création du socket serveur
        ServerSocket serveur = new ServerSocket(8080);

        System.out.println("Serveur web démarré sur le port 8080");

        // 2. Attente des connexions des clients
        while (true) {

            // 3. Acceptation d'une connexion
            Socket client = serveur.accept();

            System.out.println("Un client est connecté.");

            // 4. Préparation de la réponse HTTP
            String reponse = "HTTP/1.1 200 OK\r\n" +
                    "Content-Type: text/html; charset=UTF-8\r\n" +
                    "Connection: close\r\n" +
                    "\r\n" +
                    "<html>" +
                    "<body>" +
                    "<h1>Hello World !</h1>" +
                    "<p>Mon premier serveur web en Java.</p>" +
                    "</body>" +
                    "</html>";

            // 5. Envoi de la réponse au navigateur
            OutputStream sortie = client.getOutputStream();

            sortie.write(reponse.getBytes("UTF-8"));
            sortie.flush();

            // 6. Fermeture de la connexion client
            client.close();
        }
    }
}
