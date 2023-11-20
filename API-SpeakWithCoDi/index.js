require('dotenv').config()
const bodyParser = require('body-parser')
const express = require('express')
const cors = require('cors')
const { speakWithCodi } = require('./codi')
const doc = require('./doc')
const { archives } = require('./archives')

const app = express()
app.use(bodyParser.json())
app.use(cors())
app.use(bodyParser.urlencoded({ extended: true }))

app.get('/', (req, res) => {
    res.json(doc)
})
app.post('/speak-with-codi', speakWithCodi)
app.get('/fragmented-memory/:id', (req, res) => {
    const { id } = req.params
    switch (id) {
        case '6f4be33b-67fd-4247-8519-e73371fb01fb': return res.json({ "triva": "Dans quel établissement la société secrète Skull and Bones est-elle traditionnellement basée?" }) //Yale
        case 'ee0e9d14-c8e1-4440-b2a4-2cf63b5dba14': return res.json({ "triva": "Le nom du personnage joué par John Travolta qui recrute un hacker pour une opération de cybercriminalité?" }) //Gabriel Shear
        case '187770ac-7c8e-4d09-af38-352c85fd0800': return res.json({ "triva": "Hack the ?" })   //Planet
        case 'c07a93c9-84b1-4550-8a45-a8579aacc254': return res.json({ "triva": "Quelle est la réponse à la vie, l'univers et le reste?" }) //42
        default: return res.json({ "triva": "Je n'arrive pas à reconstituer ce fragment de mémoire." })
    }
})
app.get('/archives/:id', (req, res) => {
    const { id } = req.params
    switch (id) {
        case 'cst{w3b_5ss3mbly_1s_n0t_s3cr3t}': return res.json({ 'CoDi': archives[1907] })
        case 'cst{Un_4utr3_fl4g}': return res.json({ 'CoDi': archives[1909] })
        case 'cst{brav0_cec1_3st_un_fl4g_m4is_4uss1_un_pwd}': return res.json({ 'CoDi': archives[1909] })
        case 'cst{av3c_le_b0n_0ut1l_3t_l3_pwd_c3st_f4c1l3}': return res.json({ 'CoDi': archives[1909] })
        case 'cst{l4_brut3_ou_la_cl3}': return res.json({ 'CoDi': archives[1910] })
        case 'cst{l3_cl0ch3r_cl0ch3}': return res.json({ 'CoDi': archives[1910] })
        case 'cst{h1d3_y0ur_k3y5_0959345f876170d4dd044d6aa2b9ee5a}': return res.json({ 'CoDi': archives[1938] })
        case 'cst{032649}': return res.json({ 'CoDi': archives[1939] })
        case 'cst{c4ch3_d4ns_l3_ch4mps}': return res.json({ 'CoDi': archives[1940] })
        case 'cst{str1ng_c4ch3_d4ns_sc4nd4t4_f4c1le}': return res.json({ 'CoDi': archives[1940] })
        case 'cst{apr3s_l3oI_f4ut_s4v01r_r3g4rd3r}': return res.json({ 'CoDi': archives[1940] })
        case 'cst{d3l3t3d_f1l3s_3x1st5}': return res.json({ 'CoDi': archives[1944] })
        case 'cst{c4rv1ng_4rch1v3s}': return res.json({ 'CoDi': archives[1944] })
        case 'cst{c0ncur3nc3_sq1_s5ns_tr5ns5ct10n}': return res.json({ 'CoDi': archives[1960] })
        case 'cst{d3n01s1ng-s1gn4l-1s-h5rd-3e0ugh}': return res.json({ 'CoDi': archives[1968] })
        case 'cst{w3_4r3_4ll_c0nn3ct3d}': return res.json({ 'CoDi': archives[1992] })
        case 'cst{th1s_t0ken_1s_4n_1nv1t4t10n_t0_0ur_s0ciety}': return res.json({ 'CoDi': archives[1992] })
        case 'cst{l3s_1cmp_0nt_plus13ur5_p4yl04dzz}': return res.json({ 'CoDi': archives[1992] })
        case 'cst{s3rv3r_1s_l34k1ng}': return res.json({ 'CoDi': archives[1992] })
        case 'cst{dh3_1s_s3cur3}': return res.json({ 'CoDi': archives[1992] })
        default: return res.json({ "CoDi": "Je n'arrive pas à reconstituer ce fragment de mémoire." })
    }
});

const port = Number(process.env.PORT ?? 4000)

app.listen(port, () => {
    console.log(`Api listening on port ${port}`)
})
