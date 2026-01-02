use ratatui::style::Color;

pub mod automatic;
pub mod manipulations;
pub mod manual;
pub mod statistics;

pub enum CurrentScreen {
    Menu,
    Sandbox,
    Exiting,
}

pub enum SandboxMode {
    Automatic,
    Manual,
}

pub enum InputState {
    EnteringText,
    EnteringBitIndex,
    ShowingResult,
}

#[derive(Clone)]
pub struct ColoredText {
    pub text: String,
    pub color: Color,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub current_mode: Option<SandboxMode>,
    pub messages: Vec<String>,
    pub colored_messages: Vec<Vec<ColoredText>>,
    pub input_buffer: String,
    pub input_state: Option<InputState>,
    pub original_text: String,
    pub bit_index: Option<usize>,

    pub input_cursor_position: usize,
    pub input_scroll_offset: usize,

    pub output_scroll_offset: usize,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Menu,
            current_mode: None,
            messages: Vec::new(),
            colored_messages: Vec::new(),
            input_buffer: String::new(),
            input_state: None,
            original_text: String::new(),
            bit_index: None,
            input_cursor_position: 0,
            input_scroll_offset: 0,
            output_scroll_offset: 0,
        }
    }

    pub fn handle_input(&mut self, c: char) {
        if let Some(InputState::EnteringText | InputState::EnteringBitIndex) = &self.input_state {
            self.input_buffer.insert(self.input_cursor_position, c);
            self.input_cursor_position += 1;
            self.adjust_input_scroll();
        }
    }

    pub fn handle_backspace(&mut self) {
        if self.input_state.is_some() && self.input_cursor_position > 0 {
            self.input_cursor_position -= 1;
            self.input_buffer.remove(self.input_cursor_position);
            self.adjust_input_scroll();
        }
    }

    pub fn handle_delete(&mut self) {
        if self.input_state.is_some() && self.input_cursor_position < self.input_buffer.len() {
            self.input_buffer.remove(self.input_cursor_position);
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.input_cursor_position > 0 {
            self.input_cursor_position -= 1;
            self.adjust_input_scroll();
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.input_cursor_position < self.input_buffer.len() {
            self.input_cursor_position += 1;
            self.adjust_input_scroll();
        }
    }

    pub fn move_cursor_home(&mut self) {
        self.input_cursor_position = 0;
        self.input_scroll_offset = 0;
    }

    pub fn move_cursor_end(&mut self) {
        self.input_cursor_position = self.input_buffer.len();
        self.adjust_input_scroll();
    }

    fn adjust_input_scroll(&mut self) {
        let visible_width = 80; // TODO remove hardcode

        if self.input_cursor_position < self.input_scroll_offset {
            self.input_scroll_offset = self.input_cursor_position;
        } else if self.input_cursor_position >= self.input_scroll_offset + visible_width {
            self.input_scroll_offset = self.input_cursor_position.saturating_sub(visible_width - 1);
        }
    }

    pub fn adjust_input_scroll_with_width(&mut self, width: usize) {
        let visible_width = width.saturating_sub(2);

        if self.input_cursor_position < self.input_scroll_offset {
            self.input_scroll_offset = self.input_cursor_position;
        } else if self.input_cursor_position >= self.input_scroll_offset + visible_width {
            self.input_scroll_offset = self.input_cursor_position.saturating_sub(visible_width - 1);
        }
    }

    pub fn scroll_output_up(&mut self) {
        self.output_scroll_offset = self.output_scroll_offset.saturating_sub(1);
    }

    pub fn scroll_output_down(&mut self) {
        if self.output_scroll_offset < self.messages.len().saturating_sub(1) {
            self.output_scroll_offset += 1;
        }
    }

    pub fn scroll_output_page_up(&mut self, page_size: usize) {
        self.output_scroll_offset = self.output_scroll_offset.saturating_sub(page_size);
    }

    pub fn scroll_output_page_down(&mut self, page_size: usize) {
        let max_offset = self.messages.len().saturating_sub(page_size);
        self.output_scroll_offset = (self.output_scroll_offset + page_size).min(max_offset);
    }

    pub fn get_visible_input(&self, width: usize, prompt_len: usize) -> (String, usize, String) {
        let indicator = if self.input_buffer.is_empty() {
            String::new()
        } else if self.input_scroll_offset > 0
            || self.input_buffer.len() > width.saturating_sub(2 + prompt_len)
        {
            format!(
                "<{}/{}>",
                self.input_cursor_position + 1,
                self.input_buffer.len()
            )
        } else {
            String::new()
        };

        let indicator_len = if indicator.is_empty() {
            0
        } else {
            indicator.len() + 1
        };
        let available_width = width.saturating_sub(2 + prompt_len + indicator_len);

        if available_width == 0 {
            return (String::new(), 0, indicator);
        }

        let _end_pos = (self.input_scroll_offset + available_width).min(self.input_buffer.len()); // TODO think about unused variables
        let visible_text = self
            .input_buffer
            .chars()
            .skip(self.input_scroll_offset)
            .take(available_width)
            .collect();

        let cursor_in_visible = self
            .input_cursor_position
            .saturating_sub(self.input_scroll_offset);

        (visible_text, cursor_in_visible, indicator)
    }

    pub fn submit_input(&mut self) {
        if let Some(state) = &self.input_state
            && let Some(sandbox_mode) = &self.current_mode {
                match sandbox_mode {
                    SandboxMode::Manual => match state {
                        InputState::EnteringText => {
                            if !self.input_buffer.is_empty() {
                                self.original_text = self.input_buffer.clone();
                                self.input_buffer.clear();
                                self.input_cursor_position = 0;
                                self.input_scroll_offset = 0;
                                self.input_state = Some(InputState::EnteringBitIndex);
                                let first_10: String =
                                    self.original_text.chars().clone().take(10).collect();
                                let last_10: String = self
                                    .original_text
                                    .chars()
                                    .clone()
                                    .rev()
                                    .take(10)
                                    .collect::<Vec<_>>()
                                    .into_iter()
                                    .rev()
                                    .collect();

                                self.messages.push(format!(
                                    "Text in format first..last: {}...{}",
                                    first_10, last_10
                                )); // TODO remove harcode, do better format 

                                self.messages.push(
                                    "Enter bit index to flip (or press Enter for no flip):"
                                        .to_string(),
                                );
                                self.scroll_to_bottom();
                            }
                        }
                        InputState::EnteringBitIndex => {
                            let bit_idx = if self.input_buffer.is_empty() {
                                None
                            } else {
                                self.input_buffer.parse().ok()
                            };

                            self.bit_index = bit_idx;
                            self.input_buffer.clear();
                            self.input_cursor_position = 0;
                            self.input_scroll_offset = 0;
                            self.input_state = Some(InputState::ShowingResult);
                            self.process_manual_input();
                            self.scroll_to_bottom();
                        }
                        InputState::ShowingResult => {
                            self.input_state = Some(InputState::EnteringText);
                            self.messages.clear();
                            self.colored_messages.clear();
                            self.output_scroll_offset = 0;
                            self.messages.push("Enter string to hash:".to_string());
                        }
                    },
                    SandboxMode::Automatic => match state {
                        InputState::EnteringText => {
                            if !self.input_buffer.is_empty() {
                                self.original_text = self.input_buffer.clone();
                                self.input_buffer.clear();
                                self.input_cursor_position = 0;
                                self.input_scroll_offset = 0;
                                let first_10: String =
                                    self.original_text.chars().clone().take(10).collect();
                                let last_10: String = self
                                    .original_text
                                    .chars()
                                    .clone()
                                    .rev()
                                    .take(10)
                                    .collect::<Vec<_>>()
                                    .into_iter()
                                    .rev()
                                    .collect();

                                self.messages.push(format!(
                                    "Text in format first..last: {}...{}",
                                    first_10, last_10
                                )); // TODO remove harcode, do better format 

                                self.process_automatic();
                                self.input_state = Some(InputState::ShowingResult);
                                self.scroll_to_bottom();
                            }
                        }
                        InputState::ShowingResult => {
                            self.input_state = Some(InputState::EnteringText);
                            self.messages.clear();
                            self.colored_messages.clear();
                            self.output_scroll_offset = 0;
                            self.messages.push("Enter string to hash:".to_string());
                        }
                        _ => panic!("todo"),
                    },
                }
            }
    }

    fn scroll_to_bottom(&mut self) {
        if self.messages.len() > 10 {
            self.output_scroll_offset = self.messages.len().saturating_sub(10);
        }
    }

    pub fn get_input_prompt(&self) -> String {
        if let Some(state) = &self.input_state {
            match state {
                InputState::EnteringText => "Enter text: ",
                InputState::EnteringBitIndex => "Enter bit index: ",
                InputState::ShowingResult => "Press Enter to continue...",
            }
        } else {
            ""
        }
        .to_string()
    }

    fn add_colored_text_message(&mut self, colored_text: &[ColoredText]) {
        self.colored_messages.push(colored_text.to_vec());
        self.messages.push(" ".to_string());
    }

    pub fn switch_to_menu(&mut self) {
        self.current_screen = CurrentScreen::Menu;
        self.current_mode = None;
        self.input_state = None;
        self.input_buffer.clear();
        self.messages.clear();
        self.colored_messages.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_new() {
        let app = App::new();
        assert!(matches!(app.current_screen, CurrentScreen::Menu));
        assert!(app.current_mode.is_none());
        assert!(app.input_state.is_none());
        assert_eq!(app.input_buffer, "");
        assert_eq!(app.input_cursor_position, 0);
        assert_eq!(app.input_scroll_offset, 0);
        assert_eq!(app.output_scroll_offset, 0);
    }

    #[test]
    fn test_handle_input_entering_text() {
        let mut app = App::new();
        app.input_state = Some(InputState::EnteringText);

        app.handle_input('a');
        assert_eq!(app.input_buffer, "a");
        assert_eq!(app.input_cursor_position, 1);

        app.handle_input('b');
        assert_eq!(app.input_buffer, "ab");
        assert_eq!(app.input_cursor_position, 2);
    }

    #[test]
    fn test_handle_input_at_cursor_position() {
        let mut app = App::new();
        app.input_state = Some(InputState::EnteringText);
        app.input_buffer = "hello".to_string();
        app.input_cursor_position = 2;

        app.handle_input('X');
        assert_eq!(app.input_buffer, "heXllo");
        assert_eq!(app.input_cursor_position, 3);
    }

    #[test]
    fn test_handle_input_no_state() {
        let mut app = App::new();
        app.handle_input('a');
        assert_eq!(app.input_buffer, "");
    }

    #[test]
    fn test_handle_backspace_basic() {
        let mut app = App::new();
        app.input_state = Some(InputState::EnteringText);
        app.input_buffer = "hello".to_string();
        app.input_cursor_position = 5;

        app.handle_backspace();
        assert_eq!(app.input_buffer, "hell");
        assert_eq!(app.input_cursor_position, 4);
    }

    #[test]
    fn test_handle_backspace_middle() {
        let mut app = App::new();
        app.input_state = Some(InputState::EnteringText);
        app.input_buffer = "hello".to_string();
        app.input_cursor_position = 3;

        app.handle_backspace();
        assert_eq!(app.input_buffer, "helo");
        assert_eq!(app.input_cursor_position, 2);
    }

    #[test]
    fn test_handle_backspace_at_start() {
        let mut app = App::new();
        app.input_state = Some(InputState::EnteringText);
        app.input_buffer = "hello".to_string();
        app.input_cursor_position = 0;

        app.handle_backspace();
        assert_eq!(app.input_buffer, "hello");
        assert_eq!(app.input_cursor_position, 0);
    }

    #[test]
    fn test_handle_delete_basic() {
        let mut app = App::new();
        app.input_state = Some(InputState::EnteringText);
        app.input_buffer = "hello".to_string();
        app.input_cursor_position = 0;

        app.handle_delete();
        assert_eq!(app.input_buffer, "ello");
        assert_eq!(app.input_cursor_position, 0);
    }

    #[test]
    fn test_handle_delete_middle() {
        let mut app = App::new();
        app.input_state = Some(InputState::EnteringText);
        app.input_buffer = "hello".to_string();
        app.input_cursor_position = 2;

        app.handle_delete();
        assert_eq!(app.input_buffer, "helo");
        assert_eq!(app.input_cursor_position, 2);
    }

    #[test]
    fn test_handle_delete_at_end() {
        let mut app = App::new();
        app.input_state = Some(InputState::EnteringText);
        app.input_buffer = "hello".to_string();
        app.input_cursor_position = 5;

        app.handle_delete();
        assert_eq!(app.input_buffer, "hello");
        assert_eq!(app.input_cursor_position, 5);
    }

    #[test]
    fn test_move_cursor_left() {
        let mut app = App::new();
        app.input_buffer = "hello".to_string();
        app.input_cursor_position = 5;

        app.move_cursor_left();
        assert_eq!(app.input_cursor_position, 4);

        app.move_cursor_left();
        assert_eq!(app.input_cursor_position, 3);
    }

    #[test]
    fn test_move_cursor_left_at_start() {
        let mut app = App::new();
        app.input_buffer = "hello".to_string();
        app.input_cursor_position = 0;

        app.move_cursor_left();
        assert_eq!(app.input_cursor_position, 0);
    }

    #[test]
    fn test_move_cursor_right() {
        let mut app = App::new();
        app.input_buffer = "hello".to_string();
        app.input_cursor_position = 0;

        app.move_cursor_right();
        assert_eq!(app.input_cursor_position, 1);

        app.move_cursor_right();
        assert_eq!(app.input_cursor_position, 2);
    }

    #[test]
    fn test_move_cursor_right_at_end() {
        let mut app = App::new();
        app.input_buffer = "hello".to_string();
        app.input_cursor_position = 5;

        app.move_cursor_right();
        assert_eq!(app.input_cursor_position, 5);
    }

    #[test]
    fn test_move_cursor_home() {
        let mut app = App::new();
        app.input_buffer = "hello".to_string();
        app.input_cursor_position = 5;
        app.input_scroll_offset = 10;

        app.move_cursor_home();
        assert_eq!(app.input_cursor_position, 0);
        assert_eq!(app.input_scroll_offset, 0);
    }

    #[test]
    fn test_move_cursor_end() {
        let mut app = App::new();
        app.input_buffer = "hello".to_string();
        app.input_cursor_position = 0;

        app.move_cursor_end();
        assert_eq!(app.input_cursor_position, 5);
    }

    #[test]
    fn test_scroll_output_up() {
        let mut app = App::new();
        app.output_scroll_offset = 5;

        app.scroll_output_up();
        assert_eq!(app.output_scroll_offset, 4);

        app.scroll_output_up();
        assert_eq!(app.output_scroll_offset, 3);
    }

    #[test]
    fn test_scroll_output_up_at_top() {
        let mut app = App::new();
        app.output_scroll_offset = 0;

        app.scroll_output_up();
        assert_eq!(app.output_scroll_offset, 0);
    }

    #[test]
    fn test_scroll_output_down() {
        let mut app = App::new();
        app.messages = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        app.output_scroll_offset = 0;

        app.scroll_output_down();
        assert_eq!(app.output_scroll_offset, 1);

        app.scroll_output_down();
        assert_eq!(app.output_scroll_offset, 2);
    }

    #[test]
    fn test_scroll_output_down_at_bottom() {
        let mut app = App::new();
        app.messages = vec!["a".to_string(), "b".to_string()];
        app.output_scroll_offset = 1;

        app.scroll_output_down();
        assert_eq!(app.output_scroll_offset, 1);
    }

    #[test]
    fn test_scroll_output_page_up() {
        let mut app = App::new();
        app.output_scroll_offset = 20;

        app.scroll_output_page_up(10);
        assert_eq!(app.output_scroll_offset, 10);

        app.scroll_output_page_up(15);
        assert_eq!(app.output_scroll_offset, 0);
    }

    #[test]
    fn test_scroll_output_page_down() {
        let mut app = App::new();
        app.messages = vec!["msg".to_string(); 50];
        app.output_scroll_offset = 0;

        app.scroll_output_page_down(10);
        assert_eq!(app.output_scroll_offset, 10);

        app.scroll_output_page_down(10);
        assert_eq!(app.output_scroll_offset, 20);
    }

    #[test]
    fn test_scroll_output_page_down_respects_max() {
        let mut app = App::new();
        app.messages = vec!["msg".to_string(); 15];
        app.output_scroll_offset = 0;

        app.scroll_output_page_down(10);
        assert_eq!(app.output_scroll_offset, 5);
    }

    #[test]
    fn test_get_visible_input_no_scroll() {
        let mut app = App::new();
        app.input_buffer = "hello".to_string();
        app.input_cursor_position = 3;
        app.input_scroll_offset = 0;

        let (visible, cursor, indicator) = app.get_visible_input(100, 10);
        assert_eq!(visible, "hello");
        assert_eq!(cursor, 3);
        assert_eq!(indicator, "");
    }

    #[test]
    fn test_get_visible_input_with_scroll() {
        let mut app = App::new();
        app.input_buffer = "hello world this is a very long string".to_string();
        app.input_cursor_position = 20;
        app.input_scroll_offset = 10;

        let (_visible, cursor, _indicator) = app.get_visible_input(30, 5);
        assert_eq!(cursor, 10);
    }

    #[test]
    fn test_get_visible_input_empty() {
        let app = App::new();
        let (visible, cursor, indicator) = app.get_visible_input(100, 10);
        assert_eq!(visible, "");
        assert_eq!(cursor, 0);
        assert_eq!(indicator, "");
    }

    #[test]
    fn test_get_input_prompt_entering_text() {
        let mut app = App::new();
        app.input_state = Some(InputState::EnteringText);
        assert_eq!(app.get_input_prompt(), "Enter text: ");
    }

    #[test]
    fn test_get_input_prompt_entering_bit_index() {
        let mut app = App::new();
        app.input_state = Some(InputState::EnteringBitIndex);
        assert_eq!(app.get_input_prompt(), "Enter bit index: ");
    }

    #[test]
    fn test_get_input_prompt_showing_result() {
        let mut app = App::new();
        app.input_state = Some(InputState::ShowingResult);
        assert_eq!(app.get_input_prompt(), "Press Enter to continue...");
    }

    #[test]
    fn test_get_input_prompt_no_state() {
        let app = App::new();
        assert_eq!(app.get_input_prompt(), "");
    }

    #[test]
    fn test_switch_to_menu() {
        let mut app = App::new();
        app.current_screen = CurrentScreen::Sandbox;
        app.current_mode = Some(SandboxMode::Manual);
        app.input_state = Some(InputState::EnteringText);
        app.input_buffer = "test".to_string();
        app.messages.push("message".to_string());

        app.switch_to_menu();
        assert!(matches!(app.current_screen, CurrentScreen::Menu));
        assert!(app.current_mode.is_none());
        assert!(app.input_state.is_none());
        assert_eq!(app.input_buffer, "");
        assert!(app.messages.is_empty());
    }

    #[test]
    fn test_submit_input_manual_entering_text() {
        let mut app = App::new();
        app.current_mode = Some(SandboxMode::Manual);
        app.input_state = Some(InputState::EnteringText);
        app.input_buffer = "hello".to_string();

        app.submit_input();
        assert_eq!(app.original_text, "hello");
        assert_eq!(app.input_buffer, "");
        assert_eq!(app.input_cursor_position, 0);
        assert_eq!(app.input_scroll_offset, 0);
        assert!(matches!(
            app.input_state,
            Some(InputState::EnteringBitIndex)
        ));
    }

    #[test]
    fn test_submit_input_manual_entering_bit_index() {
        let mut app = App::new();
        app.current_mode = Some(SandboxMode::Manual);
        app.input_state = Some(InputState::EnteringBitIndex);
        app.input_buffer = "42".to_string();
        app.original_text = "hello".to_string();

        app.submit_input();
        assert_eq!(app.bit_index, Some(42));
        assert_eq!(app.input_buffer, "");
        assert!(matches!(app.input_state, Some(InputState::ShowingResult)));
    }

    #[test]
    fn test_submit_input_manual_empty_bit_index() {
        let mut app = App::new();
        app.current_mode = Some(SandboxMode::Manual);
        app.input_state = Some(InputState::EnteringBitIndex);
        app.input_buffer = "".to_string();
        app.original_text = "hello".to_string();

        app.submit_input();
        assert_eq!(app.bit_index, None);
        assert!(matches!(app.input_state, Some(InputState::ShowingResult)));
    }

    #[test]
    fn test_submit_input_manual_showing_result() {
        let mut app = App::new();
        app.current_mode = Some(SandboxMode::Manual);
        app.input_state = Some(InputState::ShowingResult);
        app.messages.push("old message".to_string());
        app.output_scroll_offset = 10;

        app.submit_input();
        assert!(matches!(app.input_state, Some(InputState::EnteringText)));
        assert!(app.messages.contains(&"Enter string to hash:".to_string()));
        assert_eq!(app.output_scroll_offset, 0);
    }

    #[test]
    fn test_submit_input_automatic_entering_text() {
        let mut app = App::new();
        app.current_mode = Some(SandboxMode::Automatic);
        app.input_state = Some(InputState::EnteringText);
        app.input_buffer = "test".to_string();

        app.submit_input();
        assert_eq!(app.original_text, "test");
        assert!(matches!(app.input_state, Some(InputState::ShowingResult)));
    }

    #[test]
    fn test_submit_input_automatic_showing_result() {
        let mut app = App::new();
        app.current_mode = Some(SandboxMode::Automatic);
        app.input_state = Some(InputState::ShowingResult);
        app.messages.push("old message".to_string());

        app.submit_input();
        assert!(matches!(app.input_state, Some(InputState::EnteringText)));
        assert!(app.messages.contains(&"Enter string to hash:".to_string()));
    }

    #[test]
    fn test_submit_input_empty_text_ignored() {
        let mut app = App::new();
        app.current_mode = Some(SandboxMode::Manual);
        app.input_state = Some(InputState::EnteringText);
        app.input_buffer = "".to_string();

        app.submit_input();
        assert!(matches!(app.input_state, Some(InputState::EnteringText)));
    }

    #[test]
    fn test_adjust_input_scroll_with_width() {
        let mut app = App::new();
        app.input_buffer = "hello world this is a test".to_string();
        app.input_cursor_position = 20;

        app.adjust_input_scroll_with_width(15);
        assert!(app.input_scroll_offset > 0);
    }

    #[test]
    fn test_adjust_input_scroll_with_width_cursor_before_scroll() {
        let mut app = App::new();
        app.input_buffer = "hello world".to_string();
        app.input_cursor_position = 2;
        app.input_scroll_offset = 5;

        app.adjust_input_scroll_with_width(20);
        assert_eq!(app.input_scroll_offset, 2);
    }
}
