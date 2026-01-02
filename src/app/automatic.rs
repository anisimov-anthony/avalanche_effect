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
        hasher.update(self.original_text.as_bytes());
        let result = hasher.finalize();
        let initial_hash: String = result.iter().map(|b| format!("{:08b}", b)).collect();

        let statistics = Arc::new(Mutex::new(Vec::new()));

        let time = std::time::Instant::now();
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
        let elapsed = time.elapsed();

        let stats = statistics.lock().unwrap();
        let max_val = stats.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min_val = stats.iter().cloned().fold(f64::INFINITY, f64::min);
        let avg = stats.iter().cloned().sum::<f64>() / stats.len() as f64;

        self.messages.push("=== Summary ===".to_string());
        self.messages.push(format!("Minimum: {:.2}%", min_val));
        self.messages.push(format!("Maximum: {:.2}%", max_val));
        self.messages.push(format!("Average: {:.2}%", avg));
        self.messages
            .push(format!("Computation time: {:?}", elapsed));
        self.messages.push("Press Enter to continue...".to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch_to_automatic() {
        let mut app = App::new();
        app.original_text = "test".to_string();
        app.switch_to_automatic();

        assert!(matches!(app.current_screen, CurrentScreen::Sandbox));
        assert!(matches!(app.current_mode, Some(SandboxMode::Automatic)));
        assert!(matches!(app.input_state, Some(InputState::EnteringText)));
    }

    #[test]
    fn test_process_automatic_generates_statistics() {
        let mut app = App::new();
        app.original_text = "test".to_string();

        app.process_automatic();

        assert!(app.messages.iter().any(|m| m.contains("=== Summary ===")));
        assert!(app.messages.iter().any(|m| m.contains("Minimum:")));
        assert!(app.messages.iter().any(|m| m.contains("Maximum:")));
        assert!(app.messages.iter().any(|m| m.contains("Average:")));
        assert!(app.messages.iter().any(|m| m.contains("Computation time")));
        assert!(app.messages.iter().any(|m| m.contains("Press Enter")));
    }

    #[test]
    fn test_process_automatic_with_short_string() {
        let mut app = App::new();
        app.original_text = "a".to_string();

        app.process_automatic();

        assert!(app.messages.iter().any(|m| m.contains("Minimum:")));
        assert!(app.messages.iter().any(|m| m.contains("Maximum:")));
        assert!(app.messages.iter().any(|m| m.contains("Average:")));
    }

    #[test]
    fn test_process_automatic_statistics_values() {
        let mut app = App::new();
        app.original_text = "abc".to_string();

        app.process_automatic();

        let min_msg = app
            .messages
            .iter()
            .find(|m| m.contains("Minimum:"))
            .unwrap();
        let max_msg = app
            .messages
            .iter()
            .find(|m| m.contains("Maximum:"))
            .unwrap();
        let avg_msg = app
            .messages
            .iter()
            .find(|m| m.contains("Average:"))
            .unwrap();

        assert!(min_msg.contains("%"));
        assert!(max_msg.contains("%"));
        assert!(avg_msg.contains("%"));
    }

    #[test]
    fn test_process_automatic_processes_all_bits() {
        let mut app = App::new();
        let test_string = "hello";
        app.original_text = test_string.to_string();

        app.process_automatic();

        let avg_msg = app
            .messages
            .iter()
            .find(|m| m.contains("Average:"))
            .unwrap();

        assert!(avg_msg.contains("%"));
    }

    #[test]
    fn test_process_automatic_appends_to_messages() {
        let mut app = App::new();
        app.original_text = "test".to_string();

        let initial_count = app.messages.len();
        app.process_automatic();

        assert!(app.messages.len() > initial_count);
        assert!(app.messages.iter().any(|m| m.contains("=== Summary ===")));
    }

    #[test]
    fn test_switch_to_automatic_initializes_correctly() {
        let mut app = App::new();
        app.current_screen = CurrentScreen::Menu;
        app.original_text = "test".to_string();

        app.switch_to_automatic();

        assert!(matches!(app.current_screen, CurrentScreen::Sandbox));
        assert!(matches!(app.current_mode, Some(SandboxMode::Automatic)));
    }

    #[test]
    fn test_process_automatic_empty_string() {
        let mut app = App::new();
        app.original_text = "".to_string();

        app.process_automatic();

        assert!(app.messages.iter().any(|m| m.contains("=== Summary ===")));
    }
}
