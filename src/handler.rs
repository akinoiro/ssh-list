use crate::*;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
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
                match app.table_state.selected() {
                    Some(_) => {
                        if check_openssh() {
                            ratatui::restore();
                            execute!(stdout(), Show).ok();
                            app.connect();
                            return false
                        } else {
                            app.search();
                            app.error_text = "Failed to execute ssh command.\nIs the OpenSSH-client installed?".to_string();
                            app.show_config_popup = false;
                            app.show_error_popup = true;
                            app.app_mode = AppMode::Error;                    
                        }
                    }
                    None => ()
                }
            },
            KeyCode::Delete => {
                app.delete_connection();
                app.scroll_state = app.scroll_state.content_length(app.ssh_connections.len());
            },
            KeyCode::Char('c'|'C'|'с'|'С') => {
                app.copy_connection();
                app.scroll_state = app.scroll_state.content_length(app.ssh_connections.len());
                app.next_row();
            },
            KeyCode::Char('i'|'I'|'ш'|'Ш') => {
                app.show_config_popup = true;
                app.app_mode = AppMode::ImportExport;
            }
            KeyCode::Char('m'|'M'|'ь'|'Ь') =>
                match app.table_state.selected() {
                    Some(_) => app.app_mode = AppMode::Move,
                    None => ()
                }
            KeyCode::Char('e'|'E'|'у'|'У') => {
                match app.table_state.selected() {
                    Some(_) => {
                        app.search();
                        app.last_app_mode = AppMode::Normal;
                        app.app_mode = AppMode::Edit;
                        app.focus = Focus::ServerNameField;
                        app.selected_config_to_fields();
                        app.show_popup = true;
                    }
                    None => ()
                }
            }
            KeyCode::Char('a'|'A'|'ф'|'Ф') => {
                app.app_mode = AppMode::New;
                app.reset_fields();
                app.show_popup = true;
                app.focus = Focus::ServerNameField;
            }
            KeyCode::Char('r'|'R'|'к'|'К') => {
                match app.table_state.selected() {
                    Some(_) => {
                        app.search();
                        app.app_mode = AppMode::RunCommand;
                        app.last_app_mode = AppMode::Normal;
                        app.reset_fields();
                        app.show_run_popup = true;
                    }
                    None => ()
                }
            }
            KeyCode::Char('/') => {
                app.app_mode = AppMode::Search;
                app.focus = Focus::SearchField;
                app.search();
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
                _ => ()
            },
        },
        AppMode::Edit => match key.code {
            KeyCode::Enter => {
                app.update_connection();
                if app.last_app_mode == AppMode::Normal {app.app_mode = AppMode::Normal};
                if app.last_app_mode == AppMode::Search {
                    app.app_mode = AppMode::Search;
                    app.focus = Focus::SearchField;
                    app.search();
                };
                app.show_popup = false;
            }
            KeyCode::Esc => {
                app.show_popup = false;
                if app.last_app_mode == AppMode::Normal {app.app_mode = AppMode::Normal};
                if app.last_app_mode == AppMode::Search {
                    app.app_mode = AppMode::Search;
                    app.focus = Focus::SearchField;
                };
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
                _ => ()
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
            KeyCode::Char('i'|'I'|'ш'|'Ш')  => {
                if !parse::check_blank_sshconfig(&parse::get_sshconfig_path()) {
                    if check_openssh() {
                        parse::import_config(app);
                        app.show_config_popup = false;
                        app.app_mode = AppMode::Normal;
                        app.scroll_state = app.scroll_state.content_length(app.ssh_connections.len());
                    } else {
                        app.search();
                        app.error_text = "Failed to import ssh config.\nIs the OpenSSH-client installed?".to_string();
                        app.show_config_popup = false;
                        app.show_error_popup = true;
                        app.app_mode = AppMode::Error;                    
                    }
                } else {
                    app.search();
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
                if app.last_app_mode == AppMode::Normal {app.app_mode = AppMode::Normal};
                if app.last_app_mode == AppMode::Search {
                    app.app_mode = AppMode::Search;
                    app.focus = Focus::SearchField;
                };
            }
            _ => {}
        }
        AppMode::RunCommand =>  match key.code {
            KeyCode::Esc => {
                app.show_run_popup = false;
                if app.last_app_mode == AppMode::Normal {
                    app.app_mode = AppMode::Normal;
                    app.focus = Focus::ServerNameField;
                };
                if app.last_app_mode == AppMode::Search {
                    app.app_mode = AppMode::Search;
                    app.focus = Focus::SearchField;
                };
                app.run_input = Input::default();
            }
            KeyCode::Enter => {
                if check_openssh() {
                    ratatui::restore();
                    execute!(stdout(), Show).ok();
                    app.run_command(app.run_input.to_string());
                    return false
                } else {
                    app.error_text = "Failed to execute ssh command.\nIs the OpenSSH-client installed?".to_string();
                    app.show_run_popup = false;
                    app.show_error_popup = true;
                    app.app_mode = AppMode::Error;
                }
            },
            _ => match app.focus {
                Focus::RunField => {
                    app.run_input.handle_event(&Event::Key(key));
                }
                _ => ()
            }
        }
        AppMode::Search => match key.code {
            KeyCode::Esc => {
                app.app_mode = AppMode::Normal;
                app.search_input = Input::default();
                app.search();
            }
            KeyCode::Down | KeyCode::Tab => app.next_row(),
            KeyCode::Up | KeyCode::BackTab => app.previous_row(),
            KeyCode::Enter => {
                match app.table_state.selected() {
                    Some(_) => {
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
                    }
                    None => ()
                }
            }
            KeyCode::Char('e'|'E'|'у'|'У') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                match app.table_state.selected() {
                    Some(_) => {
                        app.search();
                        app.last_app_mode = AppMode::Search;
                        app.app_mode = AppMode::Edit;
                        app.focus = Focus::ServerNameField;
                        app.selected_config_to_fields();
                        app.show_popup = true;
                    }
                    None => ()
                }
            }
            KeyCode::Char('r'|'R'|'к'|'К') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                match app.table_state.selected() {
                    Some(_) => {
                        app.app_mode = AppMode::RunCommand;
                        app.last_app_mode = AppMode::Search;
                        app.reset_fields();
                        app.show_run_popup = true;
                    }
                    None => ()
                }
            }
            KeyCode::Delete => {
                app.delete_connection();
                app.search();
                app.scroll_state = app.scroll_state.content_length(app.ssh_connections.len());
            }
            _ => match app.focus {
                Focus::SearchField => {
                    app.search_input.handle_event(&Event::Key(key));
                    app.search();
                    app.scroll_state = app.scroll_state.content_length(app.search_index.len());
                }
                _ => ()
            }
        }
    }
    true
}
