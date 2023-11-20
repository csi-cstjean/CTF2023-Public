# Utilisez une image de PHP/Apache comme point de départ
FROM php:7.4-apache

# Copiez le code source de l'application dans le conteneur
COPY ./app /var/www/html

# Installez les dépendances PHP (ajoutez d'autres extensions si nécessaires)
RUN docker-php-ext-install mysqli

# Exposez le port 80 pour Apache
EXPOSE 80

# Commande pour démarrer Apache
CMD ["apache2-foreground"]
