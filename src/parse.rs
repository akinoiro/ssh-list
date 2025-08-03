use crate::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Clone)]
pub struct SSHConfigConnection {
    server_name: String,
    username: String,
    hostname: String,
    port: String,
    identityfile: String,
}

pub fn import_config(app: &mut App) {
    let mut sshconfig: Vec<SSHConfigConnection> = Vec::new();
    let default_output = vec!["default_output".to_string()];
    parse_from_ssh(default_output, &mut sshconfig);
    let default_output_object = sshconfig[0].clone();
    sshconfig.remove(0);

    let config = load_config();
    let names = get_names(config);
    parse_from_ssh(names, &mut sshconfig);

    compare_with_defaults(&mut sshconfig, default_output_object);
    add_to_appconfig(sshconfig,app);
    App::update_config(app);

}

pub fn get_sshconfig_path() -> PathBuf {
    let mut config_dir_pathbuf = match env::home_dir() {
        Some(path) => path,
        None => {
            ratatui::restore();
            eprintln!("Error: Could not find the home directory.");
            execute!(stdout(), Show).ok();
            std::process::exit(1);
        }
    };
    config_dir_pathbuf.push(".ssh");
    let config_dir_path = config_dir_pathbuf.display().to_string();
    match fs::create_dir_all(&config_dir_path) {
        Ok(_) => (),
        Err(text) => {
            ratatui::restore();
            eprintln!("{}: {}", config_dir_path, text);
            execute!(stdout(), Show).ok();
            std::process::exit(1);
        }
    };
    config_dir_pathbuf.push("config");
    config_dir_pathbuf
}

pub fn check_blank_sshconfig() -> bool {
    let config_path = get_sshconfig_path();
    let file_data: String = fs::read_to_string(&config_path).unwrap_or_default();
    if file_data.trim().is_empty() {
        true
    } else {
        false
    }
}

fn load_config() -> BufReader<File> {
    let config_path = get_sshconfig_path();
    let file_data = File::open(&config_path).unwrap();
    let reader: BufReader<File> = BufReader::new(file_data);
    reader
}

fn get_names(config: BufReader<File>) -> Vec<String> {
    let mut ssh_names: Vec<String> = Vec::new();
    for line_result in config.lines() {
        match line_result {
            Ok(line) => {
                let line = line.trim().replace("=", " ");
                if !line.is_empty()
                    && !line.starts_with('#')
                    && line.to_lowercase().starts_with("host ")
                {
                    let line = line.split_whitespace();
                    for part in line {
                        if !part.contains("*") && part.to_lowercase() != "host" {
                            ssh_names.push(part.to_string());
                        }
                    }
                }
            }
            Err(text) => eprintln!("Error config reading: {}", text),
        }
    }
    ssh_names
}

fn parse_from_ssh(names: Vec<String>, sshconfig: &mut Vec<SSHConfigConnection>) {
    for name in &names {
        let output = Command::new("ssh")
            .arg("-G")
            .arg(name)
            .output()
            .expect("Error");
        let output = str::from_utf8(&output.stdout).expect("Error");

        let mut connection = SSHConfigConnection {
            server_name: name.to_string(),
            username: String::new(),
            hostname: String::new(),
            port: String::new(),
            identityfile: String::new(),
        };

        for mut line in output.lines() {
            line = line.trim();
            if let Some(user) = line.strip_prefix("user ") {
                connection.username = user.to_string();
            }
            if let Some(hostname) = line.strip_prefix("hostname ") {
                connection.hostname = hostname.to_string();
            }
            if let Some(port) = line.strip_prefix("port ") {
                connection.port = port.to_string();
            }
            if line.starts_with("identityfile ") {
                let arg = line.replace("identityfile ", "-i ");
                connection.identityfile.push_str(&arg);
                connection.identityfile.push(' ');
            }
        }
        sshconfig.push(connection);
    }
}

fn compare_with_defaults(
    sshconfig: &mut Vec<SSHConfigConnection>,
    default_output_object: SSHConfigConnection,
) {
    for i in sshconfig {
        if i.identityfile == default_output_object.identityfile {
            i.identityfile = String::new();
        }
    }
}

fn add_to_appconfig(sshconfig: Vec<SSHConfigConnection>, app: &mut App) {
    for c in sshconfig {
        let import = SSHConnection {
            server_name: c.server_name,
            group_name: String::new(),
            username: c.username,
            hostname: c.hostname,
            port: c.port,
            options: c.identityfile
        };
        app.ssh_connections.push(import);
    }
}