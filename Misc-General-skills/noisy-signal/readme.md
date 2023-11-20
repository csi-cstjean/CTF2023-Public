# SETUP
docker build -t noised-signal .
docker run -p 3000:3000 noised-signal

nc localhost 3000 


# FLAG
cst{d3n01s1ng-s1gn4l-1s-h5rd-3e0ugh}

# LABEL
signal-transmission, coding, network

# CHALLENGE
Incroyable ! Une missive tout droit venue du Premier Ministre, 
vous vous doutez bien que ce n'est pas le fruit du hasard. 
La société secrète de Monnoir était indubitablement à l'origine de cette intrigue. 
Malheureusement un problème de bruit dans le réseau nous empêche de connaître la suite.

Le flag est juste là, il suffit de ce connecter en TCP au port 3000. Mais le signal est mauvais, et du bruit aléatoire vient modifier le flag.

# SOLUTION
Récupérer le flag 1000 de fois et récupérer le charactère le plus fréquent position par position.
Exemple dans le sous-dossier solution

