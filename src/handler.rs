use crate::*;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};
use tui_input::backend::crossterm::EventHandler;

pub fn handle_key_event(app: &mut App, key: KeyEvent) -> bool {
    match app.app_mode {
        AppMode::Normal => match key.code {
            KeyCode::Esc => return false,
            KeyCode::Down | KeyCode::Tab => app.next_row(),
            KeyCode::Up | KeyCode::BackTab => app.previous_row(),
            KeyCode::Enter => app.connect(),
            KeyCode::Delete => app.delete_connection(),
            KeyCode::Char('m') | KeyCode::Char('M') | KeyCode::Char('ь') | KeyCode::Char('Ь') => app.app_mode = AppMode::Move,
            KeyCode::Char('e') | KeyCode::Char('E') | KeyCode::Char('у') | KeyCode::Char('У') => {
                app.app_mode = AppMode::Edit;
                app.selected_config_to_fields();
                app.show_popup = true;
            }
            KeyCode::Char('a') | KeyCode::Char('A') | KeyCode::Char('ф') | KeyCode::Char('Ф') => {
                app.app_mode = AppMode::New;
                app.reset_fields();
                app.show_popup = true;
            }
            _ => {}
        },
        AppMode::New => match key.code {
            KeyCode::Enter => {
                app.add_connection();
                app.app_mode = AppMode::Normal;
            }
            KeyCode::Esc => {
                app.show_popup = false;
                app.app_mode = AppMode::Normal;
            }
            KeyCode::Down | KeyCode::Tab => app.focus_next_field(),
            KeyCode::Up | KeyCode::BackTab => app.focus_previous_field(),
            _ => match app.focus {
                Focus::ServerNameField => {
                    app.field_inputs.server_name_input.handle_event(&Event::Key(key));
                }
                Focus::GroupNameField => {
                    app.field_inputs.group_name_input.handle_event(&Event::Key(key));
                }
                Focus::UsernameField => {
                    app.field_inputs.username_input.handle_event(&Event::Key(key));
                }
                Focus::HostnameField => {
                    app.field_inputs.hostname_input.handle_event(&Event::Key(key));
                }
                Focus::PortField => {
                    app.field_inputs.port_input.handle_event(&Event::Key(key));
                }
                Focus::OptionsField => {
                    app.field_inputs.options_input.handle_event(&Event::Key(key));
                }
            },
        },
        AppMode::Edit => match key.code {
            KeyCode::Enter => {
                app.update_connection();
                app.app_mode = AppMode::Normal;
            }
            KeyCode::Esc => {
                app.show_popup = false;
                app.app_mode = AppMode::Normal;
            }
            KeyCode::Down | KeyCode::Tab => app.focus_next_field(),
            KeyCode::Up | KeyCode::BackTab => app.focus_previous_field(),
            _ => match app.focus {
                Focus::ServerNameField => {
                    app.field_inputs.server_name_input.handle_event(&Event::Key(key));
                }
                Focus::GroupNameField => {
                    app.field_inputs.group_name_input.handle_event(&Event::Key(key));
                }
                Focus::UsernameField => {
                    app.field_inputs.username_input.handle_event(&Event::Key(key));
                }
                Focus::HostnameField => {
                    app.field_inputs.hostname_input.handle_event(&Event::Key(key));
                }
                Focus::PortField => {
                    app.field_inputs.port_input.handle_event(&Event::Key(key));
                }
                Focus::OptionsField => {
                    app.field_inputs.options_input.handle_event(&Event::Key(key));
                }
            },
        },
        AppMode::Move => match key.code {
            KeyCode::Esc => {
                app.show_popup = false;
                app.app_mode = AppMode::Normal;
            }
            KeyCode::Down => {
                app.move_row_down();
            }
            KeyCode::Up => {
                app.move_row_up();
            }
            _ => {}
        },
    }
    true
}
