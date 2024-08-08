pub fn encrypt(input: &str, key: isize) -> String {
    cipher(input, key, true)
}

pub fn decrypt(input: &str, key: isize) -> String {
    cipher(input, key, false)
}

fn cipher(input: &str, key: isize, enc: bool) -> String {
    let key = if enc { key } else { -key };

    input
        .chars()
        .map(|ch| {
            if ch.is_ascii_uppercase() {
                ((ch as isize - 'A' as isize + key).rem_euclid(26) + 'A' as isize) as u8 as char
            } else if ch.is_ascii_lowercase() {
                ((ch as isize - 'a' as isize + key).rem_euclid(26) + 'a' as isize) as u8 as char
            } else if ch.is_ascii_digit() {
                ((ch as isize - '0' as isize + key).rem_euclid(10) + '0' as isize) as u8 as char
            } else {
                ch
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_short() {
        let result = encrypt("abc", 3);
        assert_eq!(result, "def");
    }

    #[test]
    fn test_encrypt_short_negative() {
        let result = encrypt("abc", -3);
        assert_eq!(result, "xyz");
    }

    #[test]
    fn test_encrypt_long() {
        let msg =    "If he had anything confidential to say, he wrote it in cipher, that is, by so changing the order of the letters of the alphabet, that not a word could be made out.";
        let cipher = "Pm ol ohk hufaopun jvumpkluaphs av zhf, ol dyval pa pu jpwoly, aoha pz, if zv johunpun aol vykly vm aol slaalyz vm aol hswohila, aoha uva h dvyk jvbsk il thkl vba.";
        let result = encrypt(msg, 59);
        assert_eq!(result, cipher);
    }

    #[test]
    fn test_encrypt_long_negative() {
        let msg =    "If he had anything confidential to say, he wrote it in cipher, that is, by so changing the order of the letters of the alphabet, that not a word could be made out.";
        let cipher = "By ax atw tgrmabgz vhgybwxgmbte mh ltr, ax pkhmx bm bg vbiaxk, matm bl, ur lh vatgzbgz max hkwxk hy max exmmxkl hy max teiatuxm, matm ghm t phkw vhnew ux ftwx hnm.";
        let result = encrypt(msg, -59);
        assert_eq!(result, cipher);
    }

    #[test]
    fn test_encrypt_numbers() {
        let msg = "1234567890";
        let cipher = "4567890123";
        let result = encrypt(msg, 3);
        assert_eq!(result, cipher);
    }

    #[test]
    fn test_encrypt_numbers_negative() {
        let msg = "1234567890";
        let cipher = "8901234567";
        let result = encrypt(msg, -3);
        assert_eq!(result, cipher);
    }

    #[test]
    fn test_encrypt_mixed() {
        // Mixed test
        let msg = "a1b2c3d4e5f6g7h8i9j0";
        let cipher = "x4y5z6a7b8c9d0e1f2g3";
        let result = encrypt(msg, 23);
        assert_eq!(result, cipher);
    }

    #[test]
    fn test_encrypt_mixed_negative() {
        // Mixed test
        let msg = "a1b2c3d4e5f6g7h8i9j0";
        let cipher = "d8e9f0g1h2i3j4k5l6m7";
        let result = encrypt(msg, -23);
        assert_eq!(result, cipher);
    }

    #[test]
    fn test_encrypt_large_key() {
        let msg = "a1b2c3d4e5f6g7h8i9j0";
        let cipher = "w7x8y9z0a1b2c3d4e5f6";
        let result = encrypt(msg, 8_446_744_073_709_551_606);
        assert_eq!(result, cipher);
    }

    #[test]
    fn test_encrypt_large_key_negative() {
        let msg = "a1b2c3d4e5f6g7h8i9j0";
        let cipher = "e5f6g7h8i9j0k1l2m3n4";
        let result = encrypt(msg, -8_446_744_073_709_551_606);
        assert_eq!(result, cipher);
    }

    #[test]
    fn test_decryption_short() {
        let result = decrypt("def", 3);
        assert_eq!(result, "abc");
    }

    #[test]
    fn test_decryption_short_negative() {
        let result = decrypt("def", -3);
        assert_eq!(result, "ghi");
    }

    #[test]
    fn test_decryption_long() {
        let msg =    "If he had anything confidential to say, he wrote it in cipher, that is, by so changing the order of the letters of the alphabet, that not a word could be made out.";
        let cipher = "Pm ol ohk hufaopun jvumpkluaphs av zhf, ol dyval pa pu jpwoly, aoha pz, if zv johunpun aol vykly vm aol slaalyz vm aol hswohila, aoha uva h dvyk jvbsk il thkl vba.";
        let result = decrypt(cipher, 33);
        assert_eq!(result, msg);
    }

    #[test]
    fn test_decryption_long_negative() {
        let msg =    "If he had anything confidential to say, he wrote it in cipher, that is, by so changing the order of the letters of the alphabet, that not a word could be made out.";
        let cipher = "By ax atw tgrmabgz vhgybwxgmbte mh ltr, ax pkhmx bm bg vbiaxk, matm bl, ur lh vatgzbgz max hkwxk hy max exmmxkl hy max teiatuxm, matm ghm t phkw vhnew ux ftwx hnm.";
        let result = decrypt(cipher, -33);
        assert_eq!(result, msg);
    }

    #[test]
    fn test_decryption_numbers() {
        let msg = "1234567890";
        let cipher = "4567890123";
        let result = decrypt(cipher, 53);
        assert_eq!(result, msg);
    }

    #[test]
    fn test_decryption_numbers_negative() {
        let msg = "1234567890";
        let cipher = "8901234567";
        let result = decrypt(cipher, -53);
        assert_eq!(result, msg);
    }

    #[test]
    fn test_decryption_mixed() {
        let msg = "a1b2c3d4e5f6g7h8i9j0";
        let cipher = "b8c9d0e1f2g3h4i5j6k7";
        let result = decrypt(cipher, 27);
        assert_eq!(result, msg);
    }

    #[test]
    fn test_decryption_mixed_negative() {
        let msg = "a1b2c3d4e5f6g7h8i9j0";
        let cipher = "z4a5b6c7d8e9f0g1h2i3";
        let result = decrypt(cipher, -27);
        assert_eq!(result, msg);
    }

    #[test]
    fn test_decrypt_large_key() {
        let msg = "a1b2c3d4e5f6g7h8i9j0";
        let cipher = "w7x8y9z0a1b2c3d4e5f6";
        let result = decrypt(cipher, 8_446_744_073_709_551_606);
        assert_eq!(result, msg);
    }

    #[test]
    fn test_decrypt_large_key_negative() {
        let msg = "a1b2c3d4e5f6g7h8i9j0";
        let cipher = "e5f6g7h8i9j0k1l2m3n4";
        let result = decrypt(cipher, -8_446_744_073_709_551_606);
        assert_eq!(result, msg);
    }
}
