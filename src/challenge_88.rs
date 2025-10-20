/*
    - Given a vector of integers, return the index i such that the sum of all elements to
    the left of i equals the sum of all elements to the right of i.
    - If multiple such indices exist, return the middlemost one (closest to the center).
    - If none exist, return -1.
*/

pub fn mirror_index(arr: &[i32]) -> i32 {
    let mut result: i32 = -1;
    let mut results: Vec<(i32, i32)> = Vec::new();
    let middle = arr.len() / 2;

    for (index, _e) in arr.iter().enumerate() {
        let mut cont_1 = 0;
        let mut cont_2 = 0;

        for (index_aux, e_aux) in arr.iter().enumerate() {
            if index != index_aux {
                if index < index_aux {
                    cont_1 = cont_1 + e_aux;
                } else if index > index_aux {
                    cont_2 = cont_2 + e_aux;
                }
            }
        }

        results.push((cont_1, cont_2))
    }

    for (index, r) in results.iter().enumerate() {
        let r0 = r.0.unsigned_abs();
        let r1 = r.1.unsigned_abs();

        if r0 == r1 {
            result = index as i32;
            break;
        } else {
            for (index_aux, r_aux) in results.iter().enumerate() {
                if index != index_aux {
                    if r.0 == r_aux.1 && r.1 == r_aux.0 && ((index + 1) == middle) {
                        result = index as i32;
                        break;
                    }
                }
            }
        }
    }

    println!("{result}");
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_cases() {
        assert_eq!(mirror_index(&[2, 3, -1, 8, 4]), 3);
        assert_eq!(mirror_index(&[1, 2, 3, 4, 6]), 3);
        assert_eq!(mirror_index(&[1, 1, 1, 1, 1, 1]), 2);
        assert_eq!(mirror_index(&[10, -10, 10, -10]), -1);
        assert_eq!(mirror_index(&[0]), 0);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(mirror_index(&[]), -1);
        assert_eq!(mirror_index(&[5, 5, 5, 15, 5, 5, 5]), 3);
        assert_eq!(mirror_index(&[1, 2, 3, 3, 2, 1]), 2);
        assert_eq!(mirror_index(&[-1, -1, -1, 0, 1, 1, 1]), 3);
    }
}
