use crate::*;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};
use tui_input::backend::crossterm::EventHandler;

pub fn handle_key_event(app: &mut App, key: KeyEvent) -> bool {
    match app.app_mode {
        AppMode::Normal => match key.code {
            KeyCode::Esc => {
                ratatui::restore();
                execute!(stdout(), Show).ok();
                return false
            }
            KeyCode::Down | KeyCode::Tab => app.next_row(),
            KeyCode::Up | KeyCode::BackTab => app.previous_row(),
            KeyCode::Enter => {
                if check_openssh() {
                    ratatui::restore();
                    execute!(stdout(), Show).ok();
                    app.connect();
                    return false
                } else {
                    app.error_text = "Failed to execute ssh command.\nIs the OpenSSH-client installed?".to_string();
                    app.show_config_popup = false;
                    app.show_error_popup = true;
                    app.app_mode = AppMode::Error;                    
                }
            },
            KeyCode::Delete => {
                app.delete_connection();
                app.scroll_state = app.scroll_state.content_length(app.ssh_connections.len());
            },
            KeyCode::Char('c') | KeyCode::Char('C') | KeyCode::Char('с') | KeyCode::Char('С') => {
                app.show_config_popup = true;
                app.app_mode = AppMode::ImportExport;
            }
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
                app.show_popup = false;
                app.scroll_state = app.scroll_state.content_length(app.ssh_connections.len());
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
                app.show_popup = false;
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
        AppMode::ImportExport =>  match key.code {
            KeyCode::Char('i') | KeyCode::Char('I') | KeyCode::Char('ш') | KeyCode::Char('Ш')  => {
                if !parse::check_blank_sshconfig() {
                    if check_openssh() {
                        parse::import_config(app);
                        app.show_config_popup = false;
                        app.app_mode = AppMode::Normal;
                        app.scroll_state = app.scroll_state.content_length(app.ssh_connections.len());
                    } else {
                        app.error_text = "Failed to execute ssh command.\nIs the OpenSSH-client installed?".to_string();
                        app.show_config_popup = false;
                        app.show_error_popup = true;
                        app.app_mode = AppMode::Error;                    
                    }
                } else {
                    app.error_text = "The config is empty or does not exist.".to_string();
                    app.show_config_popup = false;
                    app.show_error_popup = true;
                    app.app_mode = AppMode::Error;
                }
                
            }
            KeyCode::Esc => {
                app.show_config_popup = false;
                app.app_mode = AppMode::Normal;
            }
            _ => {}
        }
        AppMode::Error =>  match key.code {
            KeyCode::Esc => {
                app.show_error_popup = false;
                app.app_mode = AppMode::Normal;
            }
            _ => {}
        }
    }
    true
}
