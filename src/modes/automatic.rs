use crate::percent_difference;
use crate::reverse::reverse_bit;
use crate::{converters::str_to_bits, statistics};
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::sync::{Arc, Mutex};
use text_io::read;

fn handler(bits: &Vec<u8>, idx: usize, initial_hash: String, statistics: &Arc<Mutex<Vec<f64>>>) {
    let changed_input = reverse_bit(&bits, idx);

    let mut hasher = Sha256::new();
    hasher.update(&changed_input);
    let result = hasher.finalize();

    let changed_result: String = result
        .iter()
        .map(|byte| format!("{:08b}", byte)) // каждый байт → 8 бит
        .collect();

    let perc = percent_difference(&initial_hash, &changed_result);

    //println!("changed percentage: {}", stats.0);
    //minimum = minimum.min(stats.0);
    //maximum = maximum.max(stats.0);

    //println!("comparison:");
    //println!("{}", stats.1);
    //println!("{}", stats.2);

    {
        let mut stats_lock = statistics.lock().unwrap();
        stats_lock.push(perc.0);
    }
}

pub fn start_automatic() -> Vec<f64> {
    println!("Input string without whitespaces");
    let input: String = read!();
    println!("User input: {}", input);
    println!("bytes input: {:?}", input.clone().as_bytes());

    let bits = str_to_bits(&input);
    println!("bits input: {:?}", bits);

    let mut hasher = Sha256::new();
    hasher.update(&input);

    let time = std::time::Instant::now();
    let result = hasher.finalize();
    let time = time.elapsed();
    println!("initial hash computation time: {:?}", time);

    let time = std::time::Instant::now();

    let initial_hash: String = result
        .iter()
        .map(|byte| format!("{:08b}", byte)) // каждый байт → 8 бит
        .collect();

    let statistics = Arc::new(Mutex::new(Vec::new()));

    (0..bits.len())
        .collect::<Vec<_>>()
        .par_iter()
        .for_each(|&idx| handler(&bits, idx, initial_hash.clone(), &statistics));
    let time = time.elapsed();
    println!("elapsed time: {:?}", time);

    statistics.lock().unwrap().to_vec()
}
