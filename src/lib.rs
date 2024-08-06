pub fn encrypt(input: &str, key: isize) -> String {
    let modified_string: String = input.chars()
        .map(|ch: char|
            if ch.is_ascii_uppercase() {
                ((ch as isize - 'A' as isize + key) % 26 + 'A' as isize) as u8 as char
            } else if ch.is_ascii_lowercase() {
                ((ch as isize - 'a' as isize + key) % 26 + 'a' as isize) as u8 as char
            } else if ch.is_ascii_digit() {
                ((ch as isize - '0' as isize + key) % 10 + '0' as isize) as u8 as char
            } else {
                ch as u8 as char
            }
        ).collect();

    modified_string
}

pub fn decrypt(input: &str, key: isize) -> String {
    let modified_string: String = input.chars()
    .map(|ch|
        if ch.is_ascii_uppercase() {
            ((ch as isize - 'A' as isize - key + 26) % 26 + 'A' as isize) as u8 as char
        } else if ch.is_ascii_lowercase() {
            ((ch as isize - 'a' as isize - key + 26) % 26 + 'a' as isize) as u8 as char
        } else if ch.is_ascii_digit() {
            let zero = '0' as isize;
            let mut ansr = ch as isize - zero;
            ansr = ansr - key;
            ansr = ansr + 10;
            ansr = ansr % 10;
            // ansr = ansr.abs();
            ansr = ansr + zero;
            let ansr8 = ansr as u8;
            let ansrchar = ansr8 as char;
            ((ch as isize - '0' as isize - key + 10) % 10 + '0' as isize) as u8 as char
        } else {
            ch
        }
    ).collect();

    modified_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption() {
        // Short test
        let result = encrypt("abc", 3);
        assert_eq!(result, "def");

        // Long string test
        let msg =    "If he had anything confidential to say, he wrote it in cipher, that is, by so changing the order of the letters of the alphabet, that not a word could be made out.";
        let cipher = "Pm ol ohk hufaopun jvumpkluaphs av zhf, ol dyval pa pu jpwoly, aoha pz, if zv johunpun aol vykly vm aol slaalyz vm aol hswohila, aoha uva h dvyk jvbsk il thkl vba.";
        let result = encrypt(msg, 59);
        assert_eq!(result, cipher); 

        // Number test
        let msg = "1234567890";
        let cipher = "4567890123";
        let result = encrypt(msg, 3);
        assert_eq!(result, cipher);

        // Mixed test
        let msg = "a1b2c3d4e5f6g7h8i9j0";
        let cipher = "x4y5z6a7b8c9d0e1f2g3";
        let result = encrypt(msg, 23);
        assert_eq!(result, cipher);
    }

    #[test]
    fn test_decryption() {
        // // Short string test
        // let result = decrypt("def", 3);
        // assert_eq!(result, "abc");

        // // Long string test
        // let msg =    "If he had anything confidential to say, he wrote it in cipher, that is, by so changing the order of the letters of the alphabet, that not a word could be made out.";
        // let cipher = "Pm ol ohk hufaopun jvumpkluaphs av zhf, ol dyval pa pu jpwoly, aoha pz, if zv johunpun aol vykly vm aol slaalyz vm aol hswohila, aoha uva h dvyk jvbsk il thkl vba.";
        // let result = decrypt(cipher, 7);   
        // assert_eq!(result, msg); 

        // Number test
        let msg = "1234567890";
        let cipher = "4567890123";
        let result = decrypt(cipher, 23);
        assert_eq!(result, msg);

        // // Mixed test
        // let msg = "a1b2c3d4e5f6g7h8i9j0";
        // let cipher = "b2c3d4e5f6g7h8i9j0k1";    // 27
        // // let cipher = "x4y5z6a7b8c9d0e1f2g3";    // 23
        // let result = decrypt(cipher, 27);
        // assert_eq!(result, msg);
    }
}
