// serveur web simple qui retourne toujours une page HTML "hello world"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>

int main()
{
    int server;
    int client;
    struct sockaddr_in address;
    char request[1024];

    // 1. Création du socket serveur
    server = socket(AF_INET, SOCK_STREAM, 0);

    if (server < 0)
    {
        perror("Socket error");
        return 1;
    }

    address.sin_family = AF_INET;
    address.sin_addr.s_addr = INADDR_ANY;
    address.sin_port = htons(8080);

    if (bind(server, (struct sockaddr *)&address, sizeof(address)) < 0)
    {
        perror("Bind error");
        return 2;
    }

    listen(server, 5);

    printf("Web server ready on http://localhost:8080\n");

    while (1)
    {
        client = accept(server, NULL, NULL);

        if (client < 0)
        {
            perror("Accept error");
            continue;
        }

        read(client, request, sizeof(request) - 1);

        char *response =
            "HTTP/1.1 200 OK\r\n"
            "Content-Type: text/html; charset=UTF-8\r\n"
            "Connection: close\r\n"
            "\r\n"
            "<html>"
            "<body>"
            "<h1>Hello World !</h1>"
            "<p>Mon premier serveur web en C.</p>"
            "</body>"
            "</html>";
        write(client, response, strlen(response));
        close(client);
    }

    close(server);
    return 0;
}
