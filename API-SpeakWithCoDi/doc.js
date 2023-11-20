const doc = {
  "FLAG": 'cst{h1dd3n_qr_c0d3s_4r3_c00l}',
  "/speak-with-codi": [{
    DESCRIPTION: 'Cette route permet de parler avec la conscience numérisée de différents membres de la société secrète à travers une API REST. Si vous n\'êtes pas membre de la société secrète, vous devrez d\'abord prouver votre valeur en relevant des défis.',
    VERBE: 'POST',
    ROUTE: '/speak-with-codi',
    BODY: {
      grade: 'string, - Votre grade, si vous en avez un, sinon une chaîne vide',
      name: 'string, - Le nom du membre de la société secrète que vous souhaitez contacter. * Obligatoire *',
      question: 'string - La question que vous souhaitez poser au membre de la société secrète. * Obligatoire *',
    },
    RETURNS: {
      '[nom du membre qui vous répond]': 'string - La réponse à votre question'
    }
  }
  ],
  "/fragmented-memory/:id": [{
    DESCRIPTION: 'Cette route permet de récupérer un fragment de mémoire numérisé par CoDi. Les fragments de mémoire sont incomplets et/ou endommagés. Il retourne donc toujours une question.',
    VERBE: 'GET',
    ROUTE: '/fragmented-memory/:id',
    PARAMS: {
      id: 'uuid - L\'identifiant du fragment de mémoire que vous souhaitez récupérer. *Obligatoire*'
    },
    RETURNS: {
      question: 'string - La question contenue dans le fragment de mémoire'
    }
  }],
  "/archives/:id": [{
    DESCRIPTION: 'Peut-être avez vous remarqué les textes en italique dans la section Archives.  Il s\'agit de vrai pan d\'histoire du cégep de St-Jean-sur-Richelieu tel que raconté dans les livres.  Toutefois, les livres ne racontent pas tout.  Cette route permet de consulter les archives de la société secrète. Vous pourrez y lire la façon dont la société secrète a influencé le cours de l\'histoire. Dans ces récits sont cachées les réponses à bien des mystères.   Chaque archive est associée à un identifiant unique. Cet identifiant a la forme suivante : cst{xxxx...} où x est un caractère ou un chiffre. Pour trouver les identifiants des archives, vous devez résoudre les défis de la catégorie Archives.',
    VERBE: 'GET',
    ROUTE: '/archives/:id',
    PARAMS: {
      id: 'uuid - L\'identifiant de l\'archive que vous souhaitez consulter. *Obligatoire*'
    },
    RETURNS: {
      CoDi: 'string - Le contenu de l\'archive'
    }
  }]
};


module.exports = doc;