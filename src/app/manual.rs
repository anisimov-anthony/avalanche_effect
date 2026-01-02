use crate::app::*;
use sha2::{Digest, Sha256};

impl App {
    pub fn switch_to_manual(&mut self) {
        self.current_screen = CurrentScreen::Sandbox;
        self.current_mode = Some(SandboxMode::Manual);
        self.input_state = Some(InputState::EnteringText);
        self.messages.clear();
        self.colored_messages.clear();
        self.messages.push("Enter string to hash:".to_string());
    }

    pub fn process_manual_input(&mut self) {
        let original_bits = manipulations::str_to_bits(&self.original_text);

        let (modified_bits, _) = if let Some(idx) = self.bit_index {
            if idx < original_bits.len() {
                (manipulations::reverse_bit(&original_bits, idx), Some(idx))
            } else {
                self.messages
                    .push("Error: Bit index out of range".to_string());
                return;
            }
        } else {
            (original_bits.clone(), None)
        };

        let mut hasher = Sha256::new();
        hasher.update(self.original_text.as_bytes());
        let hash1 = hasher.finalize();
        let hash1: String = hash1.iter().map(|b| format!("{:08b}", b)).collect();

        let mut hasher = Sha256::new();
        hasher.update(modified_bits);
        let hash2 = hasher.finalize();
        let hash2: String = hash2.iter().map(|b| format!("{:08b}", b)).collect();

        let (percent, old_colored, new_colored) = statistics::percent_difference(&hash1, &hash2);

        self.messages.push("".to_string());
        self.messages.push("Hash comparison:".to_string());
        self.messages.push("Original hash:".to_string());
        self.add_colored_text_message(&old_colored);
        self.messages.push("New hash:".to_string());
        self.add_colored_text_message(&new_colored);
        self.messages
            .push(format!("Avalanche effect: {:.2}%", percent));
        self.messages.push("".to_string());
        self.messages.push("Press Enter to continue...".to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch_to_manual() {
        let mut app = App::new();
        app.switch_to_manual();

        assert!(matches!(app.current_screen, CurrentScreen::Sandbox));
        assert!(matches!(app.current_mode, Some(SandboxMode::Manual)));
        assert!(matches!(app.input_state, Some(InputState::EnteringText)));
        assert!(app.messages.contains(&"Enter string to hash:".to_string()));
    }

    #[test]
    fn test_process_manual_input_no_bit_flip() {
        let mut app = App::new();
        app.original_text = "test".to_string();
        app.bit_index = None;

        app.process_manual_input();

        assert!(app.messages.iter().any(|m| m.contains("Hash comparison")));
        assert!(app.messages.iter().any(|m| m.contains("Original hash")));
        assert!(app.messages.iter().any(|m| m.contains("New hash")));
        assert!(app.messages.iter().any(|m| m.contains("Avalanche effect")));
        assert!(app.messages.iter().any(|m| m.contains("Press Enter")));
    }

    #[test]
    fn test_process_manual_input_with_bit_flip() {
        let mut app = App::new();
        app.original_text = "test".to_string();
        app.bit_index = Some(5);

        app.process_manual_input();

        assert!(app.messages.iter().any(|m| m.contains("Hash comparison")));
        assert!(app.messages.iter().any(|m| m.contains("Avalanche effect")));
    }

    #[test]
    fn test_process_manual_input_bit_index_out_of_range() {
        let mut app = App::new();
        app.original_text = "a".to_string();
        app.bit_index = Some(1000);

        app.process_manual_input();

        assert!(
            app.messages
                .iter()
                .any(|m| m.contains("Bit index out of range"))
        );
    }

    #[test]
    fn test_process_manual_input_generates_different_hashes_with_bit_flip() {
        let mut app = App::new();
        app.original_text = "hello".to_string();
        app.bit_index = Some(0);

        app.process_manual_input();

        let avalanche_message = app.messages.iter().find(|m| m.contains("Avalanche effect"));
        assert!(avalanche_message.is_some());

        let percentage_str = avalanche_message.unwrap();
        assert!(percentage_str.contains("%"));
    }

    #[test]
    fn test_process_manual_input_identical_hash_without_flip() {
        let mut app = App::new();
        app.original_text = "test".to_string();
        app.bit_index = None;

        app.process_manual_input();

        let avalanche_message = app
            .messages
            .iter()
            .find(|m| m.contains("Avalanche effect"))
            .unwrap();

        assert!(avalanche_message.contains("0.") || avalanche_message.contains("%"));
    }

    #[test]
    fn test_switch_to_manual_clears_previous_state() {
        let mut app = App::new();
        app.messages.push("old message".to_string());
        app.colored_messages.push(vec![]);

        app.switch_to_manual();

        assert_eq!(app.messages.len(), 1);
        assert!(app.colored_messages.is_empty());
    }
}
