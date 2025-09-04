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

    pub(in crate::app) fn process_manual_input(&mut self) {
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
        hasher.update(&self.original_text.as_bytes());
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
