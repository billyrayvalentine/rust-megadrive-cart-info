// Implementation of JIS X 0208:1997

pub fn shift_jis_to_utf8(v: &[u8]) -> Vec<u8> {
    let mut new_bytes = Vec::new();

    let mut pos = 0;
    let len = v.len();

    while pos < len {
        match v[pos] {
            // Match yen
            0x5C => {
                new_bytes.push(0xC2);
                new_bytes.push(0xA5);
            }
            // Match underline
            0x7E => {
                new_bytes.push(0xE2);
                new_bytes.push(0x80);
                new_bytes.push(0xBE);
            }
            // Ascii
            0x00..=0x7F => new_bytes.push(v[pos]),

            // Double byte
            // 0x00-0x3F is never used
            // 0x7F is never used
            // TODO catch index error if double byte has not second byte

            // Katakana
            0x81 => {
                match v[pos + 1] {
                    0x5B => {
                        new_bytes.push(0xE3);
                        new_bytes.push(0x83);
                        new_bytes.push(0xBC);
                        pos += 1;
                    }
                    _ => {}
                };
            }

            0x83 => {
                match v[pos + 1] {
                    0x40..=0x5E => {
                        new_bytes.push(0xE3);
                        new_bytes.push(0x82);
                        new_bytes.push(v[pos + 1] + 0x61);
                        pos += 1;
                    }
                    0x5F..=0x7E => {
                        new_bytes.push(0xE3);
                        new_bytes.push(0x83);
                        new_bytes.push(v[pos + 1] + 0x21);
                        pos += 1;
                    }
                    0x80..=0x96 => {
                        new_bytes.push(0xE3);
                        new_bytes.push(0x83);
                        new_bytes.push(v[pos + 1] + 0x20);
                        pos += 1;
                    }

                    _ => {}
                };
            }

            // Anything that has not been caught is unknown
            // push Unicode Character 'REPLACEMENT CHARACTER' (U+FFFD)
            _ => {
                new_bytes.push(0xEF);
                new_bytes.push(0xBF);
                new_bytes.push(0x8D);
            }
        };
        pos += 1;
    }
    new_bytes
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn shift_jis_ascii() {
        // Test ASCII
        assert_eq!(vec![0x20], shift_jis_to_utf8(&[0x20]));
        assert_eq!(
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".to_string(),
            String::from_utf8_lossy(&shift_jis_to_utf8(
                String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz").as_bytes()
            ))
        );
        assert_eq!(
            "1234567890".to_string(),
            String::from_utf8_lossy(&shift_jis_to_utf8(String::from("1234567890").as_bytes()))
        );
    }

    #[test]
    fn shift_jis_yen() {
        // Test yen
        assert_eq!(
            "¥".to_string(),
            String::from_utf8_lossy(&shift_jis_to_utf8(&[0x5C]))
        );
    }

    #[test]
    fn shift_jis_overline() {
        // Test overline
        assert_eq!(
            "‾".to_string(),
            String::from_utf8_lossy(&shift_jis_to_utf8(&[0x7E]))
        );
    }

    #[test]
    fn shift_jis_empty_error() {
        // test behaviour if we pass an empty array slice
        assert_eq!(Vec::<u8>::new(), shift_jis_to_utf8(&[]));
    }

    #[test]
    // Test Katakana
    fn shift_jis_katakana_double_byte_ki() {
        // A 0x82.. value
        assert_eq!(
            "キ".to_string(),
            String::from_utf8_lossy(&shift_jis_to_utf8(&[0x83, 0x4C]))
        );
    }

    #[test]
    fn shift_jis_katakana_double_byte_zu() {
        // A 0x82.. value
        assert_eq!(
            "ズ".to_string(),
            String::from_utf8_lossy(&shift_jis_to_utf8(&[0x83, 0x59]))
        );
    }

    #[test]
    // Test Katakana
    fn shift_jis_katakana_double_byte() {
        assert_eq!(
            "バ".to_string(),
            String::from_utf8_lossy(&shift_jis_to_utf8(&[0x83, 0x6F]))
        );
    }

    #[test]
    fn shift_jis_katakana_multiple_bytes() {
        assert_eq!(
            "バーチャレーシング".to_string(), // VirtuaRacing :)
            String::from_utf8_lossy(&shift_jis_to_utf8(&[
                0x83, 0x6f, 0x81, 0x5b, 0x83, 0x60, 0x83, 0x83, 0x83, 0x8c, 0x81, 0x5b, 0x83, 0x56,
                0x83, 0x93, 0x83, 0x4f
            ]))
        );
    }

    // Test single byte katakana
}
