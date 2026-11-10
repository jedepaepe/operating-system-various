//
// go build -o hello_server.exe hello_server.go
// 
package main

import (
    "fmt"
    "net/http"
)

func main() {

    // 1. Définition de la fonction qui traite les requêtes
    http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {

        // 2. Préparation de la réponse HTTP
        w.Header().Set("Content-Type", "text/html; charset=UTF-8")

        // 3. Envoi de la réponse au navigateur
        fmt.Fprint(w, `
            <html>
                <body>
                    <h1>Hello World !</h1>
                    <p>Mon premier serveur web en Go.</p>
                </body>
            </html>
        `)
    })

    // 4. Démarrage du serveur web sur le port 8080
    fmt.Println("Serveur web démarré sur le port 8080")

    err := http.ListenAndServe(":8080", nil)

    if err != nil {
        fmt.Println("Erreur :", err)
    }
}
