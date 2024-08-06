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
        let result = encrypt("abc", 3);
        assert_eq!(result, "def");

        let msg =    "If he had anything confidential to say, he wrote it in cipher, that is, by so changing the order of the letters of the alphabet, that not a word could be made out.";
        let cipher = "Pm ol ohk hufaopun jvumpkluaphs av zhf, ol dyval pa pu jpwoly, aoha pz, if zv johunpun aol vykly vm aol slaalyz vm aol hswohila, aoha uva h dvyk jvbsk il thkl vba.";
        let result = encrypt(msg, 7);   
        assert_eq!(result, cipher); 
    }

    #[test]
    fn test_decryption() {
        let result = decrypt("def", 3);
        assert_eq!(result, "abc");

        let msg =    "If he had anything confidential to say, he wrote it in cipher, that is, by so changing the order of the letters of the alphabet, that not a word could be made out.";
        let cipher = "Pm ol ohk hufaopun jvumpkluaphs av zhf, ol dyval pa pu jpwoly, aoha pz, if zv johunpun aol vykly vm aol slaalyz vm aol hswohila, aoha uva h dvyk jvbsk il thkl vba.";
        let result = decrypt(cipher, 7);   
        assert_eq!(result, msg); 
    }
}
