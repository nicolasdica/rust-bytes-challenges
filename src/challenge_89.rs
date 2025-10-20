/*
    - Given a compressed string like “a3b2c1”, return its expanded version “aaabbc”.

    - If this feels familiar, that’s because we tackled the reverse (String Compression) in issue 87.
*/
pub fn decompress(s: &str) -> String {
    let mut result = String::new();
    let mut result_with_split = String::new();
    let mut is_number = false;

    if s.len() > 1 {
        let first_char = s.as_bytes()[0] as char;

        if !first_char.is_numeric() {
            for ch in s.chars() {
                if !ch.is_ascii_alphabetic() && !ch.is_ascii_digit() {
                    return result;
                }

                if ch.is_numeric() {
                    is_number = true;
                    result_with_split.push(ch);
                } else {
                    if is_number == true {
                        result_with_split.push('\\');
                        is_number = false;
                    }

                    result_with_split.push(ch);
                    result_with_split.push(',');
                }
            }

            if result_with_split.len() > 1 {
                for i in result_with_split.split('\\') {
                    let parts: Vec<&str> = i.split(',').collect();
                    let asd = parts[0]
                        .repeat(parts[1].parse::<usize>().unwrap())
                        .to_string();

                    result.push_str(&asd);
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompress_basic() {
        assert_eq!(decompress("a3b2c1"), "aaabbc");
        assert_eq!(decompress("x5"), "xxxxx");
        assert_eq!(decompress("z1y2"), "zyy");
    }

    #[test]
    fn test_decompress_with_single_counts() {
        assert_eq!(decompress("a1b1c1"), "abc");
        assert_eq!(decompress("r1u1s1t1"), "rust");
    }

    #[test]
    fn test_decompress_with_multi_digit_counts() {
        assert_eq!(decompress("a10"), "aaaaaaaaaa");
        assert_eq!(decompress("b12c3"), "bbbbbbbbbbbbccc");
    }

    #[test]
    fn test_decompress_mixed_patterns() {
        assert_eq!(decompress("a2b10c1"), "aabbbbbbbbbbc"); // example with large middle
        assert_eq!(decompress("A3b1C2"), "AAAbCC"); // test case sensitivity
    }

    #[test]
    fn test_decompress_edge_cases() {
        assert_eq!(decompress(""), ""); // empty string
        assert_eq!(decompress("a0b3"), "bbb"); // handle zero count properly
        assert_eq!(decompress("q1"), "q"); // single pattern
    }

    #[test]
    fn test_decompress_invalid_inputs() {
        assert_eq!(decompress("a"), ""); // missing count
        assert_eq!(decompress("3a"), ""); // invalid order
        assert_eq!(decompress("a-1"), ""); // invalid number
    }
}
