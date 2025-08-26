use crate::{
    converters::str_to_bits,
    reverse::{print_reversed, reverse_bit},
    statistics::percent_difference,
};
use sha2::{Digest, Sha256};
use text_io::read;

pub fn start_manual() {
    println!("Input string without whitespaces");
    let input: String = read!();
    println!("User input: {}", input);
    println!("bytes input: {:?}", input.clone().as_bytes());

    let bits = str_to_bits(&input);
    println!("bits input: {:?}", bits);

    println!("Input idx in range [{},{})", 0, bits.len());
    let idx: usize = read!();
    println!("User idx: {}", idx);

    let mut hasher = Sha256::new();
    hasher.update(&input);
    let result = hasher.finalize();

    let initial_hash: String = result
        .iter()
        .map(|byte| format!("{:08b}", byte)) // каждый байт → 8 бит
        .collect();

    let reversed = reverse_bit(&bits, idx);
    print_reversed(&reversed, idx);

    let mut hasher = Sha256::new();
    hasher.update(&reversed);
    let result = hasher.finalize();

    let changed_hash: String = result
        .iter()
        .map(|byte| format!("{:08b}", byte)) // каждый байт → 8 бит
        .collect();

    let perc = percent_difference(&initial_hash, &changed_hash);

    println!("factor: {}", perc.0);
    println!("{}", perc.1);
    println!("{}", perc.2);
}
