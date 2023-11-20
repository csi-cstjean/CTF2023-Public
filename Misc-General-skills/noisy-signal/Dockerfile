# Utilisez une image de base Node.js
FROM node:14

# Définissez le répertoire de travail dans le conteneur
WORKDIR /app

# Copiez les fichiers de votre application dans le conteneur
COPY package*.json ./
COPY *.js ./

# Exposez le port 3000 pour la socket
EXPOSE 3000

# Démarrez votre application lorsque le conteneur est lancé
CMD [ "node", "index.js" ]
