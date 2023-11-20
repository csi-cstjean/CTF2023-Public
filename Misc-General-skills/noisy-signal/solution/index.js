const net = require('net');
const serverHost = 'localhost';
const serverPort = 3000;

// Get a 1000 responses from the server
function getResponses(numResponses) {
    return new Promise((resolve, reject) => {
        const socket = new net.Socket();
        const responses = [];
        socket.connect(serverPort, serverHost, () => {
            socket.on('data', (data) => {
                responses.push(data.toString());
                // Close the connection when we have enough responses, else ask for more.
                if (responses.length === numResponses) {
                    socket.end();
                    resolve(responses);
                }
                else {
                    socket.write('Request');
                }
            });
        });
    });
}

// Compute the most common character at each position
function characterByCharacterAverage(strings) {
    if (strings.length === 0 || strings[0].length === 0) {
      return null;
    }
  
    const signalLength = strings[0].length;
    let averagedResult = '';
  
    for (let i = 0; i < signalLength; i++) {
      const charCounts = new Map();
  
      for (const str of strings) {
        if (i < str.length) {
          const char = str.charAt(i);
          if (charCounts.has(char)) {
            charCounts.set(char, charCounts.get(char) + 1);
          } else {
            charCounts.set(char, 1);
          }
        }
      }
  
      let mostCommonChar = '';
      let maxCount = 0;
  
      charCounts.forEach((count, char) => {
        if (count > maxCount) {
          mostCommonChar = char;
          maxCount = count;
        }
      });
  
      averagedResult += mostCommonChar;
    }
  
    return averagedResult;
  }

// Get the responses and compute the most common character at each position
getResponses(1000).then((responses) => {
  responses.shift(); // Remove the welcome message  
  console.log(characterByCharacterAverage(responses));
}).catch((error) => { console.error('Error:', error.message); });
