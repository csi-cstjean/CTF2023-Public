# speak-with-codi-api
SpeakWithCoDi est une interface pour communiquer avec les différentes consciences via une API-Rest. La même interface permet de communiquer avec tous les membres. Chaque membre souhaite mettre à l'épreuve les jeunes hackers en leur proposant des défis.

## Fonctionnement

#### GET: "/"
Retourne les routes possibles avec des explications de base ainsi qu'un lien vers un challenge qui permet de voir une documentation complète des routes.
#### GET: "/fragmented-memory/:id/"
Retourne un trivia.  le :id est fournis par CTFd.
#### GET: "/archives/:id/"
Retourne la version de l'histoire de la société secrète.  Les :id à fournir sont les flag de la section Archives
#### POST: "/speak-with-codi/"
Permet de parler avec les membres de la société secrète.  
Une regex permet de deviner si la question est une demande de challenge. 
À mesure que le participant réussit les défis de CoDi, celui-ci aquière des grades.  
Ajouter un grade dans la requête permet d'avoir accès à des Challenges plus difficile.
###### body:
```
{
    "grade":"",
    "name":"Mgr Chaussé",
    "question": "As-tu un défi pour moi?"
}
```
