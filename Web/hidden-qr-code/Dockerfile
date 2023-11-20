FROM nginx:alpine

COPY ./src /usr/share/nginx/html

# Exposez le port 80 sur lequel le serveur NGINX écoute par défaut
EXPOSE 80

# Commande pour démarrer le serveur NGINX lorsque le conteneur est lancé
CMD ["nginx", "-g", "daemon off;"]
