function addBitNoise(inputString, noiseProbability) {
    let noisySignal = '';
  
    for (let i = 0; i < inputString.length; i++) {
      const originalChar = inputString.charAt(i);
      let noisyChar = originalChar; // Initialize noisyChar with the original character
  
      // Appliquer du bruit aléatoire au niveau des bits
      for (let j = 0; j < 8; j++) {
        if (Math.random() < noiseProbability) {
          // Inverser le bit en fonction de la probabilité de bruit
          noisyChar = String.fromCharCode(noisyChar.charCodeAt(0) ^ (1 << j));
        }
      }
  
      noisySignal += noisyChar;
    }
  
    return noisySignal;
  }

module.exports = { addBitNoise }