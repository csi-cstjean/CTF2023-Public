# Hidden link

## SETUP
```
docker build -t documentation .
docker run -d -p 8080:80 documentation
```
ou via le docker-compose
```
docker-compose up -d
```

```
http://localhost:8080
```

## CHALLENGE
Trouvez la documentation de CoDi

## SOLUTION
L'image est en faites un code QR.  Scanner l'image redirige vers l'API de CoDi.  La documention est sur le root GET:'/'

## LABEL
QR, Très facile, Web
