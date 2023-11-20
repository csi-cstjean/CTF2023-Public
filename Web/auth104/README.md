# SETUP
docker-compose up -d
server: localhost:9003

# CHALLENGE
Série de défis qui consiste à se connecter au site internet afin d'obtenir le flag de différentes manières. Maintenant, à vous de les trouver !

# LABEL
Web, Authentification

# FLAG
cst{3nv_n0t_1n_pUbl1c!}

# SOLUTION
Dans le code javascript, il y a un array avec des valeurs en hexadecimal. On decouvre que le mot environnement est cache. C'est un indice pour dire que le fichier .env est accessible par l'URL, dans lequel le flag se trouve

# AUTEUR
Macks_s