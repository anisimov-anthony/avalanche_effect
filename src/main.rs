use crate::{
    app::{App, CurrentScreen, SandboxMode},
    ui::ui,
};
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};
use std::{error::Error, io};

mod app;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| {
            if app.input_state.is_some() {
                let input_width = f.area().width as usize;
                app.adjust_input_scroll_with_width(input_width);
            }
            ui(f, app)
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }

            if key.code == KeyCode::Esc {
                match app.current_screen {
                    CurrentScreen::Sandbox => {
                        app.switch_to_menu();
                    }
                    CurrentScreen::Exiting => {
                        app.current_screen = CurrentScreen::Menu;
                    }
                    _ => {}
                }
            }

            match app.current_screen {
                CurrentScreen::Menu => match key.code {
                    KeyCode::Char('a') => {
                        app.switch_to_automatic();
                    }
                    KeyCode::Char('m') => {
                        app.switch_to_manual();
                    }
                    KeyCode::Up => {
                        app.scroll_output_up();
                    }
                    KeyCode::Down => {
                        app.scroll_output_down();
                    }
                    KeyCode::PageUp => {
                        app.scroll_output_page_up(5);
                    }
                    KeyCode::PageDown => {
                        app.scroll_output_page_down(5);
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    _ => {}
                },
                CurrentScreen::Sandbox => {
                    if app.input_state.is_some() {
                        match key.code {
                            KeyCode::Char(c) => {
                                app.handle_input(c);
                            }

                            KeyCode::Backspace => {
                                app.handle_backspace();
                            }
                            KeyCode::Delete => {
                                app.handle_delete();
                            }

                            KeyCode::Left => {
                                app.move_cursor_left();
                            }
                            KeyCode::Right => {
                                app.move_cursor_right();
                            }
                            KeyCode::Home => {
                                app.move_cursor_home();
                            }
                            KeyCode::End => {
                                app.move_cursor_end();
                            }

                            KeyCode::Up => {
                                app.scroll_output_up();
                            }
                            KeyCode::Down => {
                                app.scroll_output_down();
                            }
                            KeyCode::PageUp => {
                                app.scroll_output_page_up(5);
                            }
                            KeyCode::PageDown => {
                                app.scroll_output_page_down(5);
                            }

                            KeyCode::Enter => {
                                app.submit_input();
                            }
                            _ => {}
                        }
                    } else {
                        match key.code {
                            KeyCode::Char('s') => match app.current_mode {
                                Some(SandboxMode::Automatic) => {
                                    app.switch_to_manual();
                                }
                                Some(SandboxMode::Manual) => {
                                    app.switch_to_automatic();
                                }
                                None => {
                                    app.switch_to_manual();
                                }
                            },
                            KeyCode::Enter => {
                                if let Some(SandboxMode::Automatic) = app.current_mode {
                                    app.switch_to_automatic();
                                }
                            }

                            KeyCode::Up => {
                                app.scroll_output_up();
                            }
                            KeyCode::Down => {
                                app.scroll_output_down();
                            }
                            KeyCode::PageUp => {
                                app.scroll_output_page_up(5);
                            }
                            KeyCode::PageDown => {
                                app.scroll_output_page_down(5);
                            }
                            _ => {}
                        }
                    }
                }
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') => {
                        app.current_screen = CurrentScreen::Menu;
                    }
                    _ => {}
                },
            }
        }
    }
}
