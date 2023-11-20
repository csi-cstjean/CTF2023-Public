import express from 'express';

const port = 10000;
const app = express();

app.get('/', (req, res) => {
    res.send('Congrats ! Here is your flag : cst{wh4t_4_c00k1e!}');
});

app.listen(port, () => {console.log("Api is up on port " + port)});