FROM mariadb:latest

# Variables d'environnement pour configurer la base de données
ENV MYSQL_ROOT_PASSWORD=root_password_here_1111111111tsratarstarstarsta
ENV MYSQL_DATABASE=CoDi
ENV MYSQL_USER=user
ENV MYSQL_PASSWORD=password

# Copier le script SQL dans le conteneur
COPY init.sql /docker-entrypoint-initdb.d/

# Définir le port par défaut (facultatif, par défaut c'est 3306)
EXPOSE 3306
