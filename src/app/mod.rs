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
        if let Some(state) = &self.input_state {
            match state {
                InputState::EnteringText | InputState::EnteringBitIndex => {
                    self.input_buffer.insert(self.input_cursor_position, c);
                    self.input_cursor_position += 1;
                    self.adjust_input_scroll();
                }
                _ => {}
            }
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
        if let Some(state) = &self.input_state {
            if let Some(sandbox_mode) = &self.current_mode {
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
