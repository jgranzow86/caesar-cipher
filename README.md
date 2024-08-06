# Rusty Caesar Cipher

## This is a simple Caesar Cipher written for the Rust programing language. This will encrypt letters and numbers only. it will also maintain the case of letters. special characters are not encrypted and will show as they were in the original message.


- ### Encrypting a message
To encrypt a message you call the encrypt function and send two arguments. The first is the plain text message you want encrypted and the second argument is the shift key you want to use for encrypting the message. The function will return the encrypted text as a string.

- ### Decrypting a message
To decrypt a message you call the decrypt function and send two arguments. The first is the encrypted message you want to decrypt and the second is the shif key used when encrypting the message. The function will return the decrypted text as a string.