use crate::app::*;
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::sync::{Arc, Mutex};

impl App {
    pub fn switch_to_automatic(&mut self) {
        self.current_screen = CurrentScreen::Sandbox;
        self.current_mode = Some(SandboxMode::Automatic);
        self.input_state = Some(InputState::EnteringText);
        self.messages.clear();
        self.colored_messages.clear();
        self.messages.push("Enter string to hash:".to_string());
        self.process_automatic();
    }
    pub fn process_automatic(&mut self) {
        let original_bits = manipulations::str_to_bits(&self.original_text);

        let mut hasher = Sha256::new();
        hasher.update(&self.original_text.as_bytes());
        let result = hasher.finalize();
        let initial_hash: String = result.iter().map(|b| format!("{:08b}", b)).collect();

        let statistics = Arc::new(Mutex::new(Vec::new()));

        (0..original_bits.len()).into_par_iter().for_each(|idx| {
            let changed_bits = manipulations::reverse_bit(&original_bits, idx);

            let changed_bytes = manipulations::bits_to_bytes(&changed_bits);
            let mut hasher = Sha256::new();
            hasher.update(&changed_bytes);
            let result = hasher.finalize();
            let changed_hash: String = result.iter().map(|b| format!("{:08b}", b)).collect();

            let (percent, ..) = statistics::percent_difference(&initial_hash, &changed_hash);

            let mut stats_lock = statistics.lock().unwrap();
            stats_lock.push(percent);
        });

        let stats = statistics.lock().unwrap();
        let max_val = stats.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min_val = stats.iter().cloned().fold(f64::INFINITY, f64::min);
        let avg = stats.iter().cloned().sum::<f64>() / stats.len() as f64;

        self.messages.push("=== Summary ===".to_string());
        self.messages.push(format!("Minimum: {:.2}%", min_val));
        self.messages.push(format!("Maximum: {:.2}%", max_val));
        self.messages.push(format!("Average: {:.2}%", avg));
        self.messages.push("Press Enter to continue...".to_string());
    }
}
