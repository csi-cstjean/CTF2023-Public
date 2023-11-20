const { GRADE } = require('./grade.js')

const CODI = {
    ELDERS: [
        {
            NAME: process.env.NAME1,
            ANSWER: {
                ELDER_UNKNOWN: 'Je regrette, mais la personne à qui vous désirez vous adresser ne fait point partie de notre assemblée.',
                QUESTION_UNKNOWN: 'Je m\'excuse, mais je ne saurais répondre à cette interrogation tant que vous n\'aurez point fait montre de votre mérite.',
                NO_QUESTION: 'Sans interrogation, point de réplique.',
                INVALID_GRADE: 'Ne feignez point d\'endosser une identité qui ne vous est point propre.',
                REQUEST_CHALLENGE: {
                    GRADE_REQUIRED: GRADE.NONE,
                    ANSWER: [
                        'Très bien, je vais vous soumettre un défi.',
                        'Vous savez je suis un membre fondateur de cette société secrète.',
                        'Au début, il fallait être vigilant, nous devions encrypter chacune de nos discussions.',
                        'Voyons si vous auriez fait long feu dans ces années là:',
                    ],
                    LINK: '籬籼籽粄类簼籬籾类籼簺籿簼籨籼簹籬籲簼籽粂粆'
                },
                REQUEST_SECOND_CHALLENGE: {
                    GRADE_REQUIRED: GRADE.OBSERVATEUR,
                    ANSWER: [
                        'Vous avez le sens de l\'observation, c\'est tout à votre honneur.',
                        'Vous savez voir par delà les encryptions du siècle précédent.',
                        'Voyons si vous saurez faire de même avec les encryptions de ce siècle-ci:',
                    ],
                    LINK: [
                        '0f161312015e1916381a5f0301561436035d0b5457071c13',
                        'Le message a été xor avec une clé que seuls les vrais membres de la société connaissent.',
                        'En fait, les initiés connaissent surtout la clé sous cette forme :',
                        '23d6e2be089bed49550f30814e08ad60',
                    ]
                },
            }
        },
        {
            NAME: process.env.NAME2,
            ANSWER: {
                ELDER_UNKNOWN: 'Cette personne n\'est pas là, mec.',
                QUESTION_UNKNOWN: 'Ces trucs sont super compliqués, tu comprends pas vraiment tout ça, c\'est normal.',
                NO_QUESTION: 'Si tu veux avancer, faut que tu poses des questions, c\'est important.',
                INVALID_GRADE: 'Ton grade c\'est n\importe quoi, mec.',
                REQUEST_CHALLENGE: {
                    GRADE_REQUIRED: GRADE.NONE,
                    ANSWER: 'Hey, tu veux tenter de me prouver que t\'a ce qui faut? Passes les examens d\'arithmétiques, et on verra après :',
                    LINK: process.env.URL10
                }
            }
        },
        {
            NAME: process.env.NAME3,
            ANSWER: {
                ELDER_UNKNOWN: 'Malheureusement, la conscience de cette personne n\'a pas encore été numérisée, mais je vais voir ce que je peux faire.',
                QUESTION_UNKNOWN: 'En privé, je vous répondrais volontiers, mais en présence de tous, c\'est un peu compliqué.',
                NO_QUESTION: 'Pourquoi ne pas me poser une question pour commencer ?',
                INVALID_GRADE: 'Vous avez de l\'ambition, c\'est bien, mais ce grade que vous mentionnez... disons qu\'il est un peu inhabituel.',
                REQUEST_CHALLENGE: {
                    GRADE_REQUIRED: GRADE.NONE,
                    ANSWER: 'Très bien, voici un petit défi pour vous :',
                    LINK: process.env.URL20
                },
                REQUEST_SECOND_CHALLENGE: {
                    GRADE_REQUIRED: GRADE.ETUDIANT,
                    ANSWER: 'Ah, je vois que vous avez déjà résolu le premier défi. Voici le suivant :',
                    LINK: process.env.URL21
                }
            }

        },
        {
            NAME: process.env.NAME4,
            ANSWER: {
                ELDER_UNKNOWN: "Je ne saurais vous fournir la moindre information concernant cette entité. La notion m'est inconnue.",
                QUESTION_UNKNOWN: "Je crains que les préoccupations de ce genre ne sollicitent guère mon attention. Je vous exhorte à revenir lorsque vous aurez acquis la dignité de ma considération.",
                NO_QUESTION: "Je m'interroge quant à vos intentions. Que puis-je faire pour vous, précisément ?",
                INVALID_GRADE: "Vos manœuvres sont astucieuses, toutefois, je vous conseille de veiller à la validité du grade lors de vos futures tentatives.",
                REQUEST_CHALLENGE: {
                    GRADE_REQUIRED: GRADE.NONE,
                    ANSWER: "Vous voulez un défi? J'avais activé l'ancêtre du Captcha pour protéger l'accès à l'ancien réseau de l'ARPANET. Est-ce que vous êtes en mesure d'y accéder avec la puissance des ordinateurs d'aujourd'hui ? :",
                    LINK: process.env.URL30
                },
                REQUEST_SECOND_CHALLENGE: {
                    GRADE_REQUIRED: GRADE.NOVICE,
                    ANSWER: ['Je constate que vous avez traversé mon captcha. Bravo! Peut-être que vous pouvez m\'aider avec un autre problème :',
                        'La base de données des consciences est corrompue.  Je pense que le mot "flag" brise quelque chose.  Il faudrait que je puisse l\'ignorer dans ma recherche.',
                    ],
                    LINK: process.env.URL31
                }
            }
        }
    ],
    ENDING: {
        CHALLENGE: {
            GRADE_REQUIRED: GRADE.INITIE,
            ANSWER: [
                'Cher Hacker,',
                `Vous êtes prêt pour la révélation. Nous, les 4 Immortels, sommes ceux qui se cachent derrière tout cela.  C'est bien notre œuvre qui a fait naître le portail mystérieux du Web ce matin, et c'est bien nous qui vous avons attiré votre curiosité afin de vous pousser à pirater notre portail. Notre but ultime est de mettre à l'épreuve les talents des jeunes prodiges afin de dénicher les futurs membres de notre société secrète.`,
                `Si vous parvenez à relever ce défi avec succès, vous obtiendrez en récompense le code d'accès à notre société secrète. C'est à vous de décider si vous êtes prêt à prouver votre valeur et à rejoindre notre monde mystérieux. Le destin vous attend, cher hacker.`,
                `Bien à vous, Les 4 Immortels`
            ],
            LINK: process.env.FINAL
        },
    }
}

/**
 * POST /codi
 * BODY: { 
 * grade: string, - Grade of the user, if you have one, else empty string
 * name: string, - Name of the elder
 * question: string - Ask a question to the elder
 * }
 * @param {*} req 
 * @param {*} res 
 */
function speakWithCodi(req, res) {
    const grade = req.body.grade || '';
    const name = req.body.name || '';
    const question = req.body.question || '';

    // Check if the elder name is one of the 4 elders
    const elder = CODI.ELDERS.find(e => e.NAME.toLowerCase().trim() === name.toLowerCase().trim());
    // If the elder is not one of the 4 elders, return one of the unknown elder answer
    if (!elder) {
        const i = Math.floor(Math.random() * CODI.ELDERS.length)
        return res.json({ CoDi: CODI.ELDERS[i].ANSWER.ELDER_UNKNOWN })
    }

    // if there is no question, return one of the unknown question answer
    if (question.trim() === '') {
        return res.json({ [elder.NAME]: elder.ANSWER.NO_QUESTION })
    }

    // Check if the question is correctly formulated
    const regexQuestion = /\b(d[eé]fi|[eé]preuve|challenge|concours|jeu|test|d[eé]fier|mettre [aà] l'[eé]preuve|lancer un challenge|tester)\b.*\?/i
    if (regexQuestion.test(question)) {
        // check if the grade is valid for second challenge
        function returnSecondChallenge(gradeRequired) {
            if (elder.ANSWER.REQUEST_SECOND_CHALLENGE.GRADE_REQUIRED === gradeRequired) {
                res.json({ [elder.NAME]: [elder.ANSWER.REQUEST_SECOND_CHALLENGE.ANSWER, elder.ANSWER.REQUEST_SECOND_CHALLENGE.LINK] })
            }
            else {
                res.json({ [elder.NAME]: elder.ANSWER.INVALID_GRADE })
            }
        }
        switch (grade) {
            case GRADE.NONE: return res.json({ [elder.NAME]: [elder.ANSWER.REQUEST_CHALLENGE.ANSWER, elder.ANSWER.REQUEST_CHALLENGE.LINK] })
            case GRADE.OBSERVATEUR: return returnSecondChallenge(GRADE.OBSERVATEUR)
            case GRADE.ETUDIANT: return returnSecondChallenge(GRADE.ETUDIANT)
            case GRADE.NOVICE: return returnSecondChallenge(GRADE.NOVICE)
            case GRADE.INITIE: return res.json({ answer: CODI.ENDING.CHALLENGE.ANSWER })
            default: return res.json({ [elder.NAME]: elder.ANSWER.INVALID_GRADE })
        }
    }
    else {
        res.json({ [elder.NAME]: elder.ANSWER.QUESTION_UNKNOWN })
    }

}

module.exports = { speakWithCodi }