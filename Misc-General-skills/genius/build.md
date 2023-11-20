# Architecture

## TCP

Mon challenge utilise le protocole TCP pour établir des connexions. Lorsqu'un participant se connecte, il est accueilli par une série de questions mathématiques qu'il doit résoudre.

## Mutlithreading

Plusieurs participants peuvent se connecter et résoudre le challenge en même temps, sans se gêner mutuellement.

## Docker

`docker build -t genius .`
`docker run -p 127.0.0.1:1234:1234 --name genius genius`

ou j'ai mis un docker compose pour simplifier :

`docker compose up -d --build`
