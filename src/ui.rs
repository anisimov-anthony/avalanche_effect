use crate::app::{App, CurrentScreen, InputState, SandboxMode};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

static LOGO_COLOR: Color = Color::Rgb(180, 220, 255);

pub fn ui(frame: &mut Frame, app: &App) {
    match app.current_screen {
        CurrentScreen::Menu => render_menu(frame, app),
        CurrentScreen::Sandbox => render_sandbox(frame, app),
        CurrentScreen::Exiting => render_exit_modal(frame),
    }
}

fn render_menu(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(3)])
        .split(frame.area());

    let logo_lines = vec![
        "",
        "",
        "  ██████╗ ██╗   ██╗ █████╗ ██╗      █████╗ ███╗   ██╗ ██████╗██╗  ██╗███████╗",
        " ██╔══██╗██║   ██║██╔══██╗██║     ██╔══██╗████╗  ██║██╔════╝██║  ██║██╔════╝",
        " ███████║██║   ██║███████║██║     ███████║██╔██╗ ██║██║     ███████║█████╗  ",
        " ██╔══██║╚██╗ ██╔╝██╔══██║██║     ██╔══██║██║╚██╗██║██║     ██╔══██║██╔══╝  ",
        " ██║  ██║ ╚████╔╝ ██║  ██║███████╗██║  ██║██║ ╚████║╚██████╗██║  ██║███████╗",
        " ╚═╝  ╚═╝  ╚═══╝  ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝╚═╝  ╚═╝╚══════╝",
        "",
        "              ███████╗███████╗███████╗███████╗ ██████╗████████╗             ",
        "              ██╔════╝██╔════╝██╔════╝██╔════╝██╔════╝╚══██╔══╝             ",
        "              █████╗  █████╗  █████╗  █████╗  ██║        ██║                ",
        "              ██╔══╝  ██╔══╝  ██╔══╝  ██╔══╝  ██║        ██║                ",
        "              ███████╗██║     ██║     ███████╗╚██████╗   ██║                ",
        "              ╚══════╝╚═╝     ╚═╝     ╚══════╝ ╚═════╝   ╚═╝                ",
        "",
    ];

    let mut logo_spans = Vec::new();
    for line in logo_lines.iter() {
        logo_spans.push(Line::from(Span::styled(
            line.to_string(),
            Style::default().fg(LOGO_COLOR),
        )));
    }

    let description_text = vec![
        Line::from("Welcome to Avalanche Effect!"),
        Line::from(""),
        Line::from("This tool demonstrates the avalanche effect in hash functions -"),
        Line::from("how small changes in input create large changes in output."),
        Line::from(""),
        Line::from("Choose your mode:"),
        Line::from("• Manual    - Enter a string and flip one chosen bit"),
        Line::from("• Automatic - Enter a string, flip all bits in turn "),
    ];

    let mut all_lines = logo_spans;
    all_lines.extend(description_text);

    let welcome_text = Text::from(all_lines);

    let welcome_paragraph = Paragraph::new(welcome_text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Welcome")
                .border_style(Style::default().fg(LOGO_COLOR)),
        );

    frame.render_widget(welcome_paragraph, chunks[0]);

    render_status_bar(frame, app, chunks[1]);
}

fn render_sandbox(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(10),   // Output area
            Constraint::Length(3), // Input area
            Constraint::Length(3), // Status bar
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(LOGO_COLOR));

    let title = Paragraph::new(Text::styled(
        "Avalanche Effect Analyzer",
        Style::default().fg(LOGO_COLOR),
    ))
    .block(title_block);

    frame.render_widget(title, chunks[0]);

    render_output_area(frame, app, chunks[1]);

    if app.input_state.is_some() {
        render_input_area(frame, app, chunks[2]);
    }

    render_status_bar(frame, app, chunks[3]);
}

fn render_output_area(frame: &mut Frame, app: &App, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();
    let mut colored_idx = 0;

    for msg in app.messages.iter() {
        if msg == " " && colored_idx < app.colored_messages.len() {
            let spans: Vec<Span> = app.colored_messages[colored_idx]
                .iter()
                .map(|ct| Span::styled(ct.text.clone(), Style::default().fg(ct.color)))
                .collect();

            lines.push(Line::from(spans));
            colored_idx += 1;
        } else {
            lines.push(Line::from(msg.clone()));
        }
    }

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Analysis Results")
                .fg(LOGO_COLOR),
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

fn render_input_area(frame: &mut Frame, app: &App, area: Rect) {
    let prompt = app.get_input_prompt();
    let prompt_len = prompt.len();
    let (visible_text, cursor_pos, indicator) =
        app.get_visible_input(area.width as usize, prompt_len);

    let mut text_spans = vec![Span::styled(prompt, Style::default().fg(LOGO_COLOR))];

    if visible_text.is_empty() && app.input_buffer.is_empty() {
        text_spans.push(Span::styled(
            "█",
            Style::default().fg(Color::White).bg(Color::Gray),
        ));
    } else if visible_text.is_empty() {
        text_spans.push(Span::styled(" ", Style::default()));
    } else {
        let chars: Vec<char> = visible_text.chars().collect();

        for (i, &ch) in chars.iter().enumerate() {
            if i == cursor_pos {
                text_spans.push(Span::styled(
                    ch.to_string(),
                    Style::default().fg(Color::Black).bg(Color::White),
                ));
            } else {
                text_spans.push(Span::styled(
                    ch.to_string(),
                    Style::default().fg(LOGO_COLOR),
                ));
            }
        }

        if cursor_pos >= chars.len() && cursor_pos == visible_text.len() {
            text_spans.push(Span::styled(
                "█",
                Style::default().fg(Color::White).bg(Color::Gray),
            ));
        }
    }

    let main_line = Line::from(text_spans);

    if indicator.is_empty() {
        let input_paragraph = Paragraph::new(main_line)
            .block(Block::default().borders(Borders::ALL).title("Input"))
            .fg(LOGO_COLOR);
        frame.render_widget(input_paragraph, area);
    } else {
        let text_width = area.width.saturating_sub(2);
        let indicator_len = indicator.len();
        let current_content_len = (prompt_len
            + visible_text.len()
            + if cursor_pos >= visible_text.len() {
                1
            } else {
                0
            }) as u16;
        let padding_needed = text_width.saturating_sub(current_content_len + indicator_len as u16);

        let mut final_spans = main_line.spans;

        if padding_needed > 0 {
            final_spans.push(Span::raw(" ".repeat(padding_needed as usize)));
        }

        final_spans.push(Span::styled(indicator, Style::default().fg(LOGO_COLOR)));

        let input_paragraph = Paragraph::new(Line::from(final_spans)).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Input")
                .fg(LOGO_COLOR),
        );
        frame.render_widget(input_paragraph, area);
    }
}

fn render_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    let current_navigation_text = match app.current_screen {
        CurrentScreen::Menu => vec![Span::styled(
            "Screen: Main Menu",
            Style::default().fg(LOGO_COLOR),
        )],
        CurrentScreen::Sandbox => vec![Span::styled(
            format!(
                "Screen: Analyzer, Mode: {}",
                app.current_mode.as_ref().map_or("Unknown", |m| match m {
                    SandboxMode::Manual => "Manual",
                    SandboxMode::Automatic => "Automatic",
                })
            ),
            Style::default().fg(LOGO_COLOR),
        )],
        CurrentScreen::Exiting => vec![Span::styled(
            "Screen: Exiting",
            Style::default().fg(LOGO_COLOR),
        )],
    };

    let mode_footer = Paragraph::new(Line::from(current_navigation_text)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(LOGO_COLOR)),
    );

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Menu => Span::styled(
                "(a) Automatic Mode / (m) Manual Mode / (q) Quit",
                Style::default().fg(LOGO_COLOR),
            ),
            CurrentScreen::Sandbox => {
                if let Some(input_state) = &app.input_state {
                    match input_state {
                        InputState::EnteringText => Span::styled(
                            "Type and Enter / Esc menu",
                            Style::default().fg(LOGO_COLOR),
                        ),
                        InputState::EnteringBitIndex => Span::styled(
                            "Enter bit index and Enter / Esc menu",
                            Style::default().fg(LOGO_COLOR),
                        ),
                        InputState::ShowingResult => Span::styled(
                            "Enter continue / ↑↓ scroll / Esc menu",
                            Style::default().fg(LOGO_COLOR),
                        ),
                    }
                } else {
                    Span::styled(
                        "(s) switch mode / ↑↓ scroll / Esc menu",
                        Style::default().fg(LOGO_COLOR),
                    )
                }
            }
            CurrentScreen::Exiting => {
                Span::styled("(y) Yes / (n) No", Style::default().fg(LOGO_COLOR))
            }
        }
    };

    let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
        .block(Block::default().borders(Borders::ALL).fg(LOGO_COLOR));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);
}

fn render_exit_modal(frame: &mut Frame) {
    frame.render_widget(Clear, frame.area());
    let popup_block = Block::default()
        .title("Exit Application")
        .borders(Borders::ALL)
        .style(Style::default().fg(LOGO_COLOR));

    let exit_text = Text::styled(
        "Are you sure you want to exit?\n\n\nPress 'y' to confirm, 'n' to cancel",
        Style::default().fg(LOGO_COLOR),
    );

    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });

    let area = centered_rect(50, 25, frame.area());
    frame.render_widget(exit_paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    #[test]
    fn test_centered_rect_basic() {
        let area = Rect::new(0, 0, 100, 100);
        let centered = centered_rect(50, 50, area);

        assert!(centered.x >= 20 && centered.x <= 30);
        assert!(centered.y >= 20 && centered.y <= 30);
        assert!(centered.width >= 45 && centered.width <= 55);
        assert!(centered.height >= 45 && centered.height <= 55);
    }

    #[test]
    fn test_centered_rect_full_size() {
        let area = Rect::new(0, 0, 100, 100);
        let centered = centered_rect(100, 100, area);

        assert_eq!(centered.width, 100);
        assert_eq!(centered.height, 100);
    }

    #[test]
    fn test_centered_rect_small() {
        let area = Rect::new(0, 0, 100, 100);
        let centered = centered_rect(10, 10, area);

        assert!(centered.width <= 12);
        assert!(centered.height <= 12);
    }

    #[test]
    fn test_ui_menu_screen() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let app = App::new();

        terminal
            .draw(|frame| {
                ui(frame, &app);
            })
            .unwrap();
    }

    #[test]
    fn test_ui_sandbox_manual_mode() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut app = App::new();
        app.switch_to_manual();

        terminal
            .draw(|frame| {
                ui(frame, &app);
            })
            .unwrap();
    }

    #[test]
    fn test_ui_sandbox_automatic_mode() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut app = App::new();
        app.original_text = "test".to_string();
        app.current_screen = CurrentScreen::Sandbox;
        app.current_mode = Some(SandboxMode::Automatic);
        app.input_state = Some(InputState::ShowingResult);

        terminal
            .draw(|frame| {
                ui(frame, &app);
            })
            .unwrap();
    }

    #[test]
    fn test_ui_exit_modal() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut app = App::new();
        app.current_screen = CurrentScreen::Exiting;

        terminal
            .draw(|frame| {
                ui(frame, &app);
            })
            .unwrap();
    }

    #[test]
    fn test_render_menu_direct() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let app = App::new();

        terminal
            .draw(|frame| {
                render_menu(frame, &app);
            })
            .unwrap();
    }

    #[test]
    fn test_render_sandbox_direct() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut app = App::new();
        app.switch_to_manual();

        terminal
            .draw(|frame| {
                render_sandbox(frame, &app);
            })
            .unwrap();
    }

    #[test]
    fn test_render_exit_modal_direct() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|frame| {
                render_exit_modal(frame);
            })
            .unwrap();
    }

    #[test]
    fn test_render_with_colored_messages() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut app = App::new();
        app.switch_to_manual();
        app.original_text = "test".to_string();
        app.bit_index = Some(0);
        app.process_manual_input();

        terminal
            .draw(|frame| {
                ui(frame, &app);
            })
            .unwrap();
    }

    #[test]
    fn test_render_with_long_input() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut app = App::new();
        app.switch_to_manual();
        app.input_buffer = "a".repeat(200);
        app.input_cursor_position = 100;

        terminal
            .draw(|frame| {
                ui(frame, &app);
            })
            .unwrap();
    }

    #[test]
    fn test_render_with_scrolled_output() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut app = App::new();
        app.switch_to_manual();
        for i in 0..50 {
            app.messages.push(format!("Message {}", i));
        }
        app.output_scroll_offset = 10;

        terminal
            .draw(|frame| {
                ui(frame, &app);
            })
            .unwrap();
    }
}
