pub fn str_to_bits(input: &str) -> Vec<u8> {
    let mut result = Vec::with_capacity(input.len() * 8);
    for byte in input.as_bytes().iter() {
        for i in (0..8).rev() {
            result.push((byte >> i) & 1);
        }
    }
    result
}

pub fn bits_to_bytes(bits: &Vec<u8>) -> Vec<u8> {
    bits.chunks(8)
        .map(|chunk| {
            chunk
                .iter()
                .enumerate()
                .fold(0u8, |acc, (i, &b)| acc | (b << (7 - i)))
        })
        .collect()
}

pub fn reverse_bit(input: &Vec<u8>, bit_idx: usize) -> Vec<u8> {
    if bit_idx >= input.len() {
        return input.to_vec();
    }
    let mut res = input.to_vec();
    res[input.len() - bit_idx - 1] = 1 - res[input.len() - bit_idx - 1];
    res
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
    fn bits_to_bytes_empty() {
        let input = vec![];
        let valid_output = vec![];
        assert_eq!(bits_to_bytes(&input), valid_output);
    }

    #[test]
    fn bits_to_bytes_basic() {
        let input = vec![0, 0, 0, 0, 0, 0, 0, 0];
        let valid_output = vec![0];
        assert_eq!(bits_to_bytes(&input), valid_output);

        let input = vec![1, 1, 1, 1, 1, 1, 1, 1];
        let valid_output = vec![255];
        assert_eq!(bits_to_bytes(&input), valid_output);

        let mut input = vec![];
        input.extend(vec![1, 0, 0, 0, 0, 0, 0, 0]);
        input.extend(vec![0, 1, 0, 0, 0, 0, 0, 0]);
        input.extend(vec![0, 0, 1, 0, 0, 0, 0, 0]);
        input.extend(vec![0, 0, 0, 1, 0, 0, 0, 0]);
        input.extend(vec![0, 0, 0, 0, 1, 0, 0, 0]);
        input.extend(vec![0, 0, 0, 0, 0, 1, 0, 0]);
        input.extend(vec![0, 0, 0, 0, 0, 0, 1, 0]);
        input.extend(vec![0, 0, 0, 0, 0, 0, 0, 1]);

        let valid_output = vec![128, 64, 32, 16, 8, 4, 2, 1];
        assert_eq!(bits_to_bytes(&input), valid_output);
    }

    #[test]
    fn bits_to_bytes_without_traling_bits() {
        let input = vec![0];
        let valid_output = vec![0];
        assert_eq!(bits_to_bytes(&input), valid_output);

        let input = vec![0, 0];
        assert_eq!(bits_to_bytes(&input), valid_output);

        let input = vec![1];
        let valid_output = vec![128];
        assert_eq!(bits_to_bytes(&input), valid_output);

        let input = vec![0, 1];
        let valid_output = vec![64];
        assert_eq!(bits_to_bytes(&input), valid_output);

        let input = vec![1, 0, 1];
        let valid_output = vec![160];
        assert_eq!(bits_to_bytes(&input), valid_output);

        let mut input = vec![1, 1, 1, 1, 1, 1, 1, 1];
        input.extend(vec![0, 1]);
        let valid_output = vec![255, 64];
        assert_eq!(bits_to_bytes(&input), valid_output);

        let mut input = vec![1, 1, 1, 1, 1, 1, 1, 1];
        input.extend(vec![0, 0, 0, 0, 0, 0, 0, 0]);
        input.extend(vec![1]);
        let valid_output = vec![255, 0, 128];
        assert_eq!(bits_to_bytes(&input), valid_output);
    }

    #[test]
    fn test_reverse_bit_empty() {
        let input_vec = vec![];
        let idx = 0;
        let valid_output = vec![];
        assert_eq!(reverse_bit(&input_vec, idx), valid_output);

        let idx = 1;
        assert_eq!(reverse_bit(&input_vec, idx), valid_output);
    }

    #[test]
    fn test_reverse_bit_basic() {
        let input_vec = vec![1, 1, 1, 1, 1, 1, 1, 1];
        let idx = 0;
        let valid_output = vec![1, 1, 1, 1, 1, 1, 1, 0];
        assert_eq!(reverse_bit(&input_vec, idx), valid_output);

        let input_vec = vec![0, 0, 0, 0, 0, 0, 0, 0];
        let idx = 2;
        let valid_output = vec![0, 0, 0, 0, 0, 1, 0, 0];
        assert_eq!(reverse_bit(&input_vec, idx), valid_output);

        let mut input_vec = vec![0, 0, 0, 0, 0, 0, 0, 0];
        input_vec.extend(vec![0, 1, 0]);
        let idx = 6;
        let mut valid_output = vec![0, 0, 0, 0, 1, 0, 0, 0];
        valid_output.extend(vec![0, 1, 0]);
        assert_eq!(reverse_bit(&input_vec, idx), valid_output);
    }
}
