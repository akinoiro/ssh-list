mod handler;
mod ui;

use color_eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Flex, Layout, Rect},
    widgets::{ScrollbarState, TableState},
};
use serde::{Deserialize, Serialize};
use shlex::split;
use std::process::Command;
use std::{env, fs};
use tui_input::Input;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct SSHConnection {
    server_name: String,
    group_name: String,
    username: String,
    hostname: String,
    port: String,
    options: String,
}

impl SSHConnection {
    const fn ref_array(&self) -> [&String; 6] {
        [
            &self.server_name,
            &self.group_name,
            &self.username,
            &self.hostname,
            &self.port,
            &self.options,
        ]
    }
}

pub struct FieldInputs {
    server_name_input: Input,
    group_name_input: Input,
    username_input: Input,
    hostname_input: Input,
    port_input: Input,
    options_input: Input,
}

#[derive(PartialEq)]
pub enum Focus {
    ServerNameField,
    GroupNameField,
    UsernameField,
    HostnameField,
    PortField,
    OptionsField,
}

pub enum AppMode {
    Normal,
    Edit,
    New,
    Move,
}

pub struct App {
    table_state: TableState,
    ssh_connections: Vec<SSHConnection>,
    scroll_state: ScrollbarState,
    show_popup: bool,
    focus: Focus,
    field_inputs: FieldInputs,
    app_mode: AppMode,
}

impl App {
    fn new() -> Self {
        let data_vec = read_config();
        Self {
            table_state: TableState::default().with_selected(0),
            scroll_state: ScrollbarState::new(data_vec.len()),
            ssh_connections: data_vec,
            show_popup: false,
            focus: Focus::ServerNameField,
            field_inputs: FieldInputs {
                server_name_input: Input::default(),
                group_name_input: Input::default(),
                username_input: Input::default(),
                hostname_input: Input::default(),
                port_input: Input::default().with_value("22".to_string()),
                options_input: Input::default(),
            },
            app_mode: AppMode::Normal,
        }
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.check_blank_config();
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            let event = event::read()?;
            if let Event::Key(key) = event {
                if key.kind == KeyEventKind::Press {
                    if handler::handle_key_event(&mut self, key) == false {
                        break;
                    }
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let vertical = Layout::vertical([Constraint::Min(5), Constraint::Length(3)]);
        let rects_v = vertical.split(frame.area());

        let horizontal = Layout::horizontal([Constraint::Min(0), Constraint::Length(3)]);
        let rects_h = horizontal.split(rects_v[0]);

        ui::render_table(self, frame, rects_h[0]);
        ui::render_scrollbar(self, frame, rects_h[1]);
        ui::render_footer(self, frame, rects_v[1]);

        if self.show_popup {
            ui::render_popup(self, frame, rects_v[0]);
        }
    }

    fn check_blank_config(&mut self) {
        if self.ssh_connections == vec![] {
            self.show_popup = true;
            self.app_mode = AppMode::New
        }
    }
    fn next_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.ssh_connections.len() - 1 {
                    i
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i);
    }

    fn previous_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    i
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i);
    }

    fn connect(&mut self) {
        if let Some(i) = self.table_state.selected() {
            ratatui::restore();
            println!(
                "Connecting to {} ({})...",
                self.ssh_connections[i].server_name, self.ssh_connections[i].group_name
            );
            Command::new("ssh")
                .arg(format!("-p {}", self.ssh_connections[i].port))
                .arg(format!(
                    "{}@{}",
                    self.ssh_connections[i].username, self.ssh_connections[i].hostname
                ))
                .args(split(&self.ssh_connections[i].options).unwrap_or_default())
                .status()
                .expect("ssh not found: please install OpenSSH client");
            std::process::exit(0);
        }
    }

    fn add_connection(&mut self) {
        let new_connection = SSHConnection {
            server_name: self.field_inputs.server_name_input.to_string(),
            group_name: self.field_inputs.group_name_input.to_string(),
            username: self.field_inputs.username_input.to_string(),
            hostname: self.field_inputs.hostname_input.to_string(),
            port: self.field_inputs.port_input.to_string(),
            options: self.field_inputs.options_input.to_string(),
        };
        self.ssh_connections.push(new_connection);
        self.update_config();
        self.show_popup = false;
        self.scroll_state = self.scroll_state.content_length(self.ssh_connections.len());
        self.reset_fields();
        self.table_state = TableState::default().with_selected(self.ssh_connections.len());
    }

    fn reset_fields(&mut self) {
        self.field_inputs = FieldInputs {
            server_name_input: Input::default(),
            group_name_input: Input::default(),
            username_input: Input::default(),
            hostname_input: Input::default(),
            port_input: Input::default().with_value("22".to_string()),
            options_input: Input::default(),
        };
        self.focus = Focus::ServerNameField;
    }

    fn update_config(&mut self) {
        let json = serde_json::to_string_pretty(&self.ssh_connections).unwrap();
        fs::write(get_config_path(), json).unwrap();
    }

    fn selected_config_to_fields(&mut self) {
        if let Some(i) = self.table_state.selected() {
            self.field_inputs.server_name_input = Input::default().with_value(self.ssh_connections[i].server_name.to_string());
            self.field_inputs.group_name_input = Input::default().with_value(self.ssh_connections[i].group_name.to_string());
            self.field_inputs.username_input = Input::default().with_value(self.ssh_connections[i].username.to_string());
            self.field_inputs.hostname_input = Input::default().with_value(self.ssh_connections[i].hostname.to_string());
            self.field_inputs.port_input = Input::default().with_value(self.ssh_connections[i].port.to_string());
            self.field_inputs.options_input = Input::default().with_value(self.ssh_connections[i].options.to_string());
        };
    }

    fn update_connection(&mut self) {
        let edited_connection = SSHConnection {
            server_name: self.field_inputs.server_name_input.to_string(),
            group_name: self.field_inputs.group_name_input.to_string(),
            username: self.field_inputs.username_input.to_string(),
            hostname: self.field_inputs.hostname_input.to_string(),
            port: self.field_inputs.port_input.to_string(),
            options: self.field_inputs.options_input.to_string(),
        };
        if let Some(i) = self.table_state.selected() {
            self.ssh_connections[i] = edited_connection;
            self.table_state = TableState::default().with_selected(i);
        }
        self.update_config();
        self.show_popup = false;
        self.reset_fields();
    }

    fn delete_connection(&mut self) {
        if let Some(i) = self.table_state.selected() {
            self.ssh_connections.remove(i);
            self.update_config();
        };
        self.scroll_state = self.scroll_state.content_length(self.ssh_connections.len());
    }

    fn focus_next_field(&mut self) {
        self.focus = match self.focus {
            Focus::ServerNameField => Focus::GroupNameField,
            Focus::GroupNameField => Focus::UsernameField,
            Focus::UsernameField => Focus::HostnameField,
            Focus::HostnameField => Focus::PortField,
            Focus::PortField => Focus::OptionsField,
            Focus::OptionsField => Focus::OptionsField,
        };
    }

    fn focus_previous_field(&mut self) {
        self.focus = match self.focus {
            Focus::ServerNameField => Focus::ServerNameField,
            Focus::GroupNameField => Focus::ServerNameField,
            Focus::UsernameField => Focus::GroupNameField,
            Focus::HostnameField => Focus::UsernameField,
            Focus::PortField => Focus::HostnameField,
            Focus::OptionsField => Focus::PortField,
        };
    }

    fn move_row_down(&mut self) {
        if let Some(i) = self.table_state.selected() {
            if i >= self.ssh_connections.len() - 1 {
                self.table_state.select(Some(i));
            } else {
                self.ssh_connections.swap(i, i + 1);
                self.table_state.select(Some(i + 1));
                self.scroll_state = self.scroll_state.position(i + 1);
            }
        }
        self.update_config()
        
    }

    fn move_row_up(&mut self) {
        if let Some(i) = self.table_state.selected() {
            if i == 0 {
                self.table_state.select(Some(i));
            } else {
                self.ssh_connections.swap(i, i - 1);
                self.table_state.select(Some(i - 1));
                self.scroll_state = self.scroll_state.position(i - 1);
            }
        }
        self.update_config()
    }
}

fn get_config_path() -> String {
    let mut config_dir_pathbuf = env::home_dir().expect("homedir detect error");
    config_dir_pathbuf.push(".ssh");
    let config_dir_path = config_dir_pathbuf.display().to_string();
    fs::create_dir_all(&config_dir_path).unwrap();
    let config_path = config_dir_path + "/ssh-list.json";
    config_path
}

fn read_config() -> Vec<SSHConnection> {
    let config_path = get_config_path();
    let file_data: String = fs::read_to_string(config_path).unwrap_or_default();
    serde_json::from_str(&file_data).unwrap_or_default()
}

pub fn popup_area(area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(21)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Length(40)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
