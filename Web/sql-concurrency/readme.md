# SETUP
docker-compose up -d
server: localhost:3306
database: CoDi
user: user
password: password

# CHALLENGE
Les travaux de construction du nouvel auditorium eurent des conséquences graves, car c'est à cet endroit précis, sous terre, que le laboratoire secret avait été érigé. 
Une procédure pour transéferer les consciences numérisées ailleurs et une procédure pour confirmer que le transfert était réussie furent mise au point par la société secrète, mais les copies se faisait constament détruire, par les travaux d'agrandissement, à mesures qu'on les faisait.  La confirmation de transfert était toujours vide.

Trouvez une façon de confirmer le transfert des données plus rapidement que la destruction de ceux-ci.

# LABEL
SQL, concurrence

# FLAG
cst{c0ncur3nc3_sq1_s5ns_tr5ns5ct10n}

# SOLUTION
Exécuter les deux procédures stockées en loop jusqu'à ce que la seconde s'éxécute entre les 2 opérations de la première pour afficher le flag.

# AUTEUR
EspressoHunter