# SETUP
docker-compose up -d

# CHALLENGE
Site web impliquant un module d'encryption/décription AES en WebAssembly.

# LABEL
webassembly, deassembly, aesEncrypt

# SOLUTION
Désassembler le module webassembly pour connaître la clé AES.  
Avec la clé on peut désencrypter le message du site web qui contient la clé.
KEY: gNGMRqqiqbu4bxOkuTV9pHdB