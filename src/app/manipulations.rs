#[allow(dead_code)]
pub fn str_to_bits(input: &str) -> Vec<u8> {
    let mut result = Vec::with_capacity(input.len() * 8);
    for byte in input.as_bytes().iter() {
        for i in (0..8).rev() {
            result.push((byte >> i) & 1);
        }
    }
    result
}

#[allow(dead_code)]
pub fn bits_to_bytes(bits: &[u8]) -> Vec<u8> {
    bits.chunks(8)
        .map(|chunk| {
            chunk
                .iter()
                .enumerate()
                .fold(0u8, |acc, (i, &b)| acc | (b << (7 - i)))
        })
        .collect()
}

#[allow(dead_code)]
pub fn reverse_bit(input: &[u8], bit_idx: usize) -> Vec<u8> {
    if bit_idx >= input.len() {
        return input.to_vec();
    }
    let mut res = input.to_vec();
    res[input.len() - bit_idx - 1] = 1 - res[input.len() - bit_idx - 1];
    res
}

#[allow(dead_code)]
fn is_binary(vec: &Vec<u8>) -> bool {
    let incorrect_bit = vec.iter().find(|&&bit| (bit != 1) && (bit != 0));

    if let Some(_) = incorrect_bit {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_to_bits_empty() {
        let input = "";
        let valid_output = vec![];
        assert_eq!(str_to_bits(input), valid_output);
    }

    #[test]
    fn str_to_bits_basic() {
        let input = "qwerty";
        let valid_output = vec![
            0, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0,
            0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1,
        ];
        assert_eq!(str_to_bits(input), valid_output);
    }

    #[test]
    fn str_to_bits_with_whitespaces() {
        let input = "rust is blazing";
        let valid_output = vec![
            0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 0,
            1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0,
            1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0,
            1, 0, 1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0,
            0, 1, 1, 1,
        ];
        assert_eq!(str_to_bits(input), valid_output);
    }

    #[test]
    fn str_to_bits_special_symbols() {
        let input = "!@#$%^&*()-_=+[]{}|;:',.<>?/`~";
        let valid_output = vec![
            0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0,
            1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0,
            1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0,
            1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1,
            1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0,
            1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1,
            1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1,
            1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0,
            0, 1, 1, 1, 1, 1, 1, 0,
        ];
        assert_eq!(str_to_bits(input), valid_output);
    }

    #[test]
    fn is_binary_empty() {
        let input = vec![];
        assert!(is_binary(&input));
    }

    #[test]
    fn is_binary_basic() {
        let input = vec![0, 1, 1, 0, 1, 1];
        assert!(is_binary(&input));

        let input = vec![0, 1, 1, 2, 1, 1];
        assert!(!is_binary(&input));
    }
}
