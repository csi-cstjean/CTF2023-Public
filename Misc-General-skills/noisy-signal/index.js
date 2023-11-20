const net = require('net');
const { addBitNoise } = require('./bitNoise.js')
const { inputSignal, noiseProbability, port } = require('./data.js')

const server = net.createServer((socket) => {
    // When a client connects, send the welcome message
    socket.write(`Incroyable ! Une missive tout droit venue du Premier Ministre, 
    vous vous doutez bien que ce n'est pas le fruit du hasard. 
    La société secrète de Monnoir était indubitablement à l'origine de cette intrigue. 
    Malheureusement un problème de bruit dans le réseau nous empêche de connaître la suite. `);

    // When the client sends anything, return a new noisyResult
    socket.on('data', () => {
        socket.write(addBitNoise(inputSignal, noiseProbability));
    });

    // When the client disconnects, log that
    socket.on('end', () => {
        console.log('Client disconnected');
    });

    // If there is an error, log that
    socket.on('error', (err) => {
        console.error(err);
    });
});

server.listen(port, () => {
    console.log(`Server listening on port ${port}`);
});
