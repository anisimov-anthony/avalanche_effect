use colored::Colorize;

pub fn reverse_bit(input: &Vec<u8>, bit_idx: usize) -> Vec<u8> {
    if bit_idx < input.len() {
        let mut res = input.clone();
        res[input.len() - bit_idx - 1] = 1 - res[input.len() - bit_idx - 1];
        return res;
    }
    panic!("idx >= len");
}

pub fn print_reversed(reversed_input: &Vec<u8>, reversed_idx: usize) {
    if reversed_idx < reversed_input.len() {
        let mut res = String::with_capacity(reversed_input.len());

        for (idx, bit_val) in reversed_input.iter().enumerate() {
            if idx != (reversed_input.len() - reversed_idx - 1) {
                res += &bit_val.to_string();
            } else {
                res += &bit_val.to_string().red().to_string();
            }
        }
        println!("reversed_input for idx {}: {}", reversed_idx, res);
    } else {
        panic!("idx >= len");
    }
}
