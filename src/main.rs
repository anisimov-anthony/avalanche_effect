use core::panic;

use crate::modes::manual::start_manual;
use modes::automatic::start_automatic;
use statistics::*;
use text_io::read;

mod converters;
mod modes;
mod reverse;
mod statistics;

fn main() {
    println!("Select the mode: automatic(a), manual(m)");
    let mode: String = read!();
    if mode == "a".to_string() {
        let stats = start_automatic();
        let max_val = stats.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min_val = stats.iter().cloned().fold(f64::INFINITY, f64::min);
        let avg = stats.iter().cloned().sum::<f64>() / stats.len() as f64;

        println!("minimum: {}", min_val);
        println!("maximum: {}", max_val);
        println!("average: {}", avg);
    } else if mode == "m" {
        start_manual();
    } else {
        panic!("The selected mode is incorrect")
    }
}
