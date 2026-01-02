use avalanche_effect::app::*;

#[test]
fn test_manual_mode_complete_workflow() {
    let mut app = App::new();

    app.switch_to_manual();
    assert!(matches!(app.current_screen, CurrentScreen::Sandbox));
    assert!(matches!(app.current_mode, Some(SandboxMode::Manual)));
    assert!(matches!(app.input_state, Some(InputState::EnteringText)));

    app.input_buffer = "hello".to_string();
    app.submit_input();

    assert_eq!(app.original_text, "hello");
    assert!(matches!(
        app.input_state,
        Some(InputState::EnteringBitIndex)
    ));

    app.input_buffer = "5".to_string();
    app.submit_input();

    assert_eq!(app.bit_index, Some(5));
    assert!(matches!(app.input_state, Some(InputState::ShowingResult)));

    app.submit_input();
    assert!(matches!(app.input_state, Some(InputState::EnteringText)));
}

#[test]
fn test_automatic_mode_complete_workflow() {
    let mut app = App::new();

    app.input_buffer = "test".to_string();
    app.original_text = "test".to_string();
    app.switch_to_automatic();

    assert!(matches!(app.current_screen, CurrentScreen::Sandbox));
    assert!(matches!(app.current_mode, Some(SandboxMode::Automatic)));

    assert!(app.messages.iter().any(|m| m.contains("=== Summary ===")));
    assert!(app.messages.iter().any(|m| m.contains("Minimum:")));
    assert!(app.messages.iter().any(|m| m.contains("Maximum:")));
    assert!(app.messages.iter().any(|m| m.contains("Average:")));
}

#[test]
fn test_switching_between_modes() {
    let mut app = App::new();

    app.switch_to_manual();
    assert!(matches!(app.current_mode, Some(SandboxMode::Manual)));

    app.switch_to_menu();
    assert!(matches!(app.current_screen, CurrentScreen::Menu));
    assert!(app.current_mode.is_none());

    app.original_text = "test".to_string();
    app.switch_to_automatic();
    assert!(matches!(app.current_mode, Some(SandboxMode::Automatic)));

    app.switch_to_menu();
    assert!(matches!(app.current_screen, CurrentScreen::Menu));
}

#[test]
fn test_input_editing_workflow() {
    let mut app = App::new();
    app.input_state = Some(InputState::EnteringText);

    app.handle_input('h');
    app.handle_input('e');
    app.handle_input('l');
    app.handle_input('l');
    app.handle_input('o');
    assert_eq!(app.input_buffer, "hello");

    app.move_cursor_left();
    app.move_cursor_left();
    app.handle_input('X');
    assert_eq!(app.input_buffer, "helXlo");

    app.move_cursor_home();
    assert_eq!(app.input_cursor_position, 0);

    app.move_cursor_end();
    assert_eq!(app.input_cursor_position, 6);

    app.handle_backspace();
    assert_eq!(app.input_buffer, "helXl");
}

#[test]
fn test_cursor_movement_with_long_input() {
    let mut app = App::new();
    app.input_state = Some(InputState::EnteringText);
    app.input_buffer = "This is a very long string that should trigger scrolling".to_string();
    app.input_cursor_position = app.input_buffer.len();

    app.move_cursor_left();
    app.move_cursor_left();
    app.move_cursor_left();

    assert_eq!(
        app.input_cursor_position,
        "This is a very long string that should trigger scrolling".len() - 3
    );

    app.move_cursor_home();
    assert_eq!(app.input_cursor_position, 0);
    assert_eq!(app.input_scroll_offset, 0);
}

#[test]
fn test_output_scrolling_workflow() {
    let mut app = App::new();

    for i in 0..20 {
        app.messages.push(format!("Message {}", i));
    }

    app.output_scroll_offset = 0;
    app.scroll_output_down();
    assert_eq!(app.output_scroll_offset, 1);

    app.scroll_output_page_down(5);
    assert_eq!(app.output_scroll_offset, 6);

    app.scroll_output_up();
    assert_eq!(app.output_scroll_offset, 5);

    app.scroll_output_page_up(3);
    assert_eq!(app.output_scroll_offset, 2);
}

#[test]
fn test_manual_mode_with_invalid_bit_index() {
    let mut app = App::new();
    app.original_text = "ab".to_string();
    app.bit_index = Some(1000);

    app.process_manual_input();

    assert!(
        app.messages
            .iter()
            .any(|m| m.contains("Bit index out of range"))
    );
}

#[test]
fn test_manual_mode_without_bit_flip() {
    let mut app = App::new();
    app.original_text = "test".to_string();
    app.bit_index = None;

    app.process_manual_input();

    let avalanche_msg = app
        .messages
        .iter()
        .find(|m| m.contains("Avalanche effect"))
        .unwrap();
    assert!(avalanche_msg.contains("0.") || avalanche_msg.contains("%"));
}

#[test]
fn test_state_transitions_manual_mode() {
    let mut app = App::new();
    app.switch_to_manual();

    assert!(matches!(app.input_state, Some(InputState::EnteringText)));

    app.input_buffer = "test".to_string();
    app.submit_input();
    assert!(matches!(
        app.input_state,
        Some(InputState::EnteringBitIndex)
    ));

    app.input_buffer = "3".to_string();
    app.submit_input();
    assert!(matches!(app.input_state, Some(InputState::ShowingResult)));

    app.submit_input();
    assert!(matches!(app.input_state, Some(InputState::EnteringText)));
}

#[test]
fn test_colored_text_generation() {
    let mut app = App::new();
    app.original_text = "test".to_string();
    app.bit_index = Some(0);

    app.process_manual_input();

    assert!(!app.colored_messages.is_empty());
}

#[test]
fn test_app_reset_on_menu_switch() {
    let mut app = App::new();
    app.switch_to_manual();
    app.input_buffer = "test".to_string();
    app.messages.push("message".to_string());

    app.switch_to_menu();

    assert_eq!(app.input_buffer, "");
    assert!(app.messages.is_empty());
    assert!(app.input_state.is_none());
    assert!(app.current_mode.is_none());
}

#[test]
fn test_get_visible_input_with_long_text() {
    let mut app = App::new();
    let long_text = "a".repeat(200);
    app.input_buffer = long_text.clone();
    app.input_cursor_position = 100;

    app.adjust_input_scroll_with_width(50);

    let (visible, _cursor, indicator) = app.get_visible_input(50, 5);

    assert!(!visible.is_empty());
    assert!(indicator.contains("101/200"));
}

#[test]
fn test_delete_vs_backspace() {
    let mut app = App::new();
    app.input_state = Some(InputState::EnteringText);
    app.input_buffer = "hello".to_string();

    app.input_cursor_position = 2;
    app.handle_delete();
    assert_eq!(app.input_buffer, "helo");
    assert_eq!(app.input_cursor_position, 2);

    app.input_cursor_position = 2;
    app.handle_backspace();
    assert_eq!(app.input_buffer, "hlo");
    assert_eq!(app.input_cursor_position, 1);
}

#[test]
fn test_automatic_mode_parallel_processing() {
    let mut app = App::new();
    app.original_text = "test string".to_string();

    app.process_automatic();

    assert!(app.messages.iter().any(|m| m.contains("Minimum:")));
    assert!(app.messages.iter().any(|m| m.contains("Maximum:")));
    assert!(app.messages.iter().any(|m| m.contains("Average:")));

    let min = app
        .messages
        .iter()
        .find(|m| m.contains("Minimum:"))
        .unwrap();
    let max = app
        .messages
        .iter()
        .find(|m| m.contains("Maximum:"))
        .unwrap();
    let avg = app
        .messages
        .iter()
        .find(|m| m.contains("Average:"))
        .unwrap();

    assert!(min.contains("%"));
    assert!(max.contains("%"));
    assert!(avg.contains("%"));
}
