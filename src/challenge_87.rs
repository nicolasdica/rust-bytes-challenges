/*
    Your task is to implement a compress_string function that compresses a string by replacing
    consecutive repeated characters with the character followed by the number of repetitions.
*/

pub fn compress_string(input: &str) -> String {
    let mut letters_with_count: Vec<(char, u32)> = Vec::new();
    let mut previous_letter: Option<char> = None;
    let mut result = String::new();
    
    if input.len() > 0 {
        // Build vec with chars and amount
        for letter in input.chars() {
            if previous_letter.is_none() {
                letters_with_count.push((letter, 1));
            } else {
                if previous_letter == Some(letter) {
                    if let Some(last) = letters_with_count.last_mut() {
                        last.1 += 1;
                    }
                } else {
                    letters_with_count.push((letter, 1));
                }
            }

            previous_letter = Some(letter)
        }

        // Format vec to string
        for letter_with_count in letters_with_count.iter() {
            let vec_as_string = format!("{}{}", letter_with_count.0, letter_with_count.1);
            result.push_str(&vec_as_string);
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(compress_string("aaabbc"), "a3b2c1");
        assert_eq!(compress_string("abcd"), "a1b1c1d1");
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(compress_string(""), "");
        assert_eq!(compress_string("a"), "a1");
        assert_eq!(compress_string("zzzzzz"), "z6");
        assert_eq!(compress_string("aabbaa"), "a2b2a2");
    }
}
