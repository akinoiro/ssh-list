use crate::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use glob::glob;
use std::path::Path;

#[derive(PartialEq, Clone)]
pub struct SSHConfigConnection {
    server_name: String,
    username: String,
    hostname: String,
    port: String,
    options: String,
    identityfile: String,
    identitiesonly: String,
    clearallforwardings: String,
    exitonforwardfailure: String,
    forwardagent: String,
    forwardx11: String,
    forwardx11trusted: String,
    forwardx11timeout: String,
    serveralivecountmax: String,
    serveraliveinterval: String,
    gatewayports: String,
    passwordauthentication: String,
    pubkeyauthentication: String,
    stricthostkeychecking: String,
    connecttimeout: String,
    controlmaster: String,
    controlpath: String,
    controlpersist: String,
    compression: String,
}

pub fn import_config(app: &mut App) {
    let mut sshconfig: Vec<SSHConfigConnection> = vec![];
    let default_output = vec!["default_output".to_string()];
    parse_from_ssh(default_output, &mut sshconfig);
    let default_output_object = sshconfig[0].clone();
    sshconfig.remove(0);

    let mut ssh_config_paths: Vec<PathBuf> = vec![];
    let mut names: Vec<String> = vec![];
    ssh_config_paths.push(get_sshconfig_path());
    let config = load_config(&ssh_config_paths[0]);
    get_includes(&mut ssh_config_paths, config);
    for ssh_config_path in ssh_config_paths {
        if !check_blank_sshconfig(&ssh_config_path) {
            let config = load_config(&ssh_config_path);
            get_names(&mut names, config);
        }
    }
    parse_from_ssh(names, &mut sshconfig);

    compare_with_defaults(&mut sshconfig, default_output_object);
    add_to_appconfig(sshconfig,app);
    App::update_config(app);
    app.table_state.select(Some(app.ssh_connections.len()));
    app.scroll_state = app.scroll_state.position(app.ssh_connections.len());
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

pub fn check_blank_sshconfig(config_path: &PathBuf) -> bool {
    let file_data: String = fs::read_to_string(&config_path).unwrap_or_default();
    if file_data.trim().is_empty() {
        true
    } else {
        false
    }
}

fn load_config(config_path: &PathBuf) -> BufReader<File> {
    let file_data = File::open(&config_path).unwrap();
    let reader: BufReader<File> = BufReader::new(file_data);
    reader
}

fn get_names(names: &mut Vec<String>, config: BufReader<File>) {
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
                            names.push(part.to_string());
                        }
                    }
                }
            }
            Err(text) => eprintln!("Error config reading: {}", text),
        }
    }
}

fn parse_from_ssh(names: Vec<String>, sshconfig: &mut Vec<SSHConfigConnection>) {
    for name in &names {
        let output = Command::new("ssh")
            .arg("-G")
            .arg(name)
            .output()
            .expect("Error");
        let output = std::str::from_utf8(&output.stdout).expect("Error");

        let mut connection = SSHConfigConnection {
            server_name: name.to_string(),
            username: String::new(),
            hostname: String::new(),
            port: String::new(),
            options: String::new(),
            identityfile: String::new(),
            identitiesonly: String::new(),
            clearallforwardings: String::new(),
            exitonforwardfailure: String::new(),
            forwardagent: String::new(),
            forwardx11: String::new(),
            forwardx11trusted: String::new(),
            forwardx11timeout: String::new(),
            serveralivecountmax: String::new(),
            serveraliveinterval: String::new(),
            gatewayports: String::new(),
            passwordauthentication: String::new(),
            pubkeyauthentication: String::new(),
            stricthostkeychecking: String::new(),
            connecttimeout: String::new(),
            controlmaster: String::new(),
            controlpath: String::new(),
            controlpersist: String::new(),
            compression: String::new(),

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
            if line.starts_with("identitiesonly ") {
                let arg = line.replace("identitiesonly ", "-o IdentitiesOnly=");
                connection.identitiesonly.push_str(&arg);
                connection.identitiesonly.push(' ');
            }
            if line.starts_with("localforward ") {
                let mut line = line.split_whitespace();
                line.next();
                let port = line.next().unwrap_or_default();
                let address = line.next().unwrap_or_default();
                connection.options.push_str(format!("-L {}:{} ", port, address).as_str());
            }
            if line.starts_with("remoteforward ") {
                if !line.contains("[socks]:0") {
                    let mut line = line.split_whitespace();
                    line.next();
                    let port = line.next().unwrap_or_default();
                    let address = line.next().unwrap_or_default();
                    connection.options.push_str(format!("-R {}:{} ", port, address).as_str());
                } else {
                    let mut line = line.split_whitespace();
                    line.next();
                    let port = line.next().unwrap_or_default();
                    connection.options.push_str(format!("-R {} ", port).as_str());
                }
                
            }
            if line.starts_with("dynamicforward ") {
                let arg = line.replace("dynamicforward ", "-D ");
                connection.options.push_str(&arg);
                connection.options.push(' ');
            }
            if line.starts_with("clearallforwardings ") {
                let arg = line.replace("clearallforwardings ", "-o ClearAllForwardings=");
                connection.clearallforwardings.push_str(&arg);
                connection.clearallforwardings.push(' ');
            }
            if line.starts_with("exitonforwardfailure ") {
                let arg = line.replace("exitonforwardfailure ", "-o ExitOnForwardFailure=");
                connection.exitonforwardfailure.push_str(&arg);
                connection.exitonforwardfailure.push(' ');
            }
            if line.starts_with("forwardagent ") {
                let arg = line.replace("forwardagent ", "-o ForwardAgent=");
                connection.forwardagent.push_str(&arg);
                connection.forwardagent.push(' ');
            }
            if line.starts_with("forwardx11 ") {
                let arg = line.replace("forwardx11 ", "-o ForwardX11=");
                connection.forwardx11.push_str(&arg);
                connection.forwardx11.push(' ');
            }
            if line.starts_with("forwardx11trusted ") {
                let arg = line.replace("forwardx11trusted ", "-o ForwardX11Trusted=");
                connection.forwardx11trusted.push_str(&arg);
                connection.forwardx11trusted.push(' ');
            }
            if line.starts_with("forwardx11timeout ") {
                let arg = line.replace("forwardx11timeout ", "-o ForwardX11Timeout=");
                connection.forwardx11timeout.push_str(&arg);
                connection.forwardx11timeout.push(' ');
            }
            if line.starts_with("serveralivecountmax ") {
                let arg = line.replace("serveralivecountmax ", "-o ServerAliveCountMax=");
                connection.serveralivecountmax.push_str(&arg);
                connection.serveralivecountmax.push(' ');
            }
            if line.starts_with("serveraliveinterval ") {
                let arg = line.replace("serveraliveinterval ", "-o ServerAliveInterval=");
                connection.serveraliveinterval.push_str(&arg);
                connection.serveraliveinterval.push(' ');
            }
            if line.starts_with("gatewayports ") {
                let arg = line.replace("gatewayports ", "-o GatewayPorts=");
                connection.gatewayports.push_str(&arg);
                connection.gatewayports.push(' ');
            }
            if line.starts_with("proxyjump ") {
                let arg = line.replace("proxyjump ", "-J ");
                connection.options.push_str(&arg);
                connection.options.push(' ');
            }
            if line.starts_with("passwordauthentication ") {
                let arg = line.replace("passwordauthentication ", "-o PasswordAuthentication=");
                connection.passwordauthentication.push_str(&arg);
                connection.passwordauthentication.push(' ');
            }
            if line.starts_with("pubkeyauthentication ") {
                let arg = line.replace("pubkeyauthentication ", "-o PubkeyAuthentication=");
                connection.pubkeyauthentication.push_str(&arg);
                connection.pubkeyauthentication.push(' ');
            }
            if line.starts_with("stricthostkeychecking ") {
                let arg = line.replace("stricthostkeychecking ", "-o StrictHostKeyChecking=");
                connection.stricthostkeychecking.push_str(&arg);
                connection.stricthostkeychecking.push(' ');
            }
            if line.starts_with("connecttimeout ") {
                let arg = line.replace("connecttimeout ", "-o ConnectTimeout=");
                connection.connecttimeout.push_str(&arg);
                connection.connecttimeout.push(' ');
            }
            if line.starts_with("controlmaster ") {
                let arg = line.replace("controlmaster ", "-o ControlMaster=");
                connection.controlmaster.push_str(&arg);
                connection.controlmaster.push(' ');
            }
            if line.starts_with("controlpath ") {
                let arg = line.replace("controlpath ", "-o ControlPath=");
                connection.controlpath.push_str(&arg);
                connection.controlpath.push(' ');
            }
            if line.starts_with("controlpersist ") {
                let arg = line.replace("controlpersist ", "-o ControlPersist=");
                connection.controlpersist.push_str(&arg);
                connection.controlpersist.push(' ');
            }
            if line.starts_with("compression ") {
                let arg = line.replace("compression ", "-o Compression=");
                connection.compression.push_str(&arg);
                connection.compression.push(' ');
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
        if i.identitiesonly == default_output_object.identitiesonly {
            i.identitiesonly = String::new();
        }
        if i.clearallforwardings == default_output_object.clearallforwardings {
            i.clearallforwardings = String::new();
        }
        if i.exitonforwardfailure == default_output_object.exitonforwardfailure {
            i.exitonforwardfailure = String::new();
        }
        if i.forwardagent == default_output_object.forwardagent {
            i.forwardagent = String::new();
        }
        if i.forwardx11 == default_output_object.forwardx11 {
            i.forwardx11 = String::new();
        }
        if i.forwardx11trusted == default_output_object.forwardx11trusted {
            i.forwardx11trusted = String::new();
        }
        if i.forwardx11timeout == default_output_object.forwardx11timeout {
            i.forwardx11timeout = String::new();
        }
        if i.serveralivecountmax == default_output_object.serveralivecountmax {
            i.serveralivecountmax = String::new();
        }
        if i.serveraliveinterval == default_output_object.serveraliveinterval {
            i.serveraliveinterval = String::new();
        }
        if i.gatewayports == default_output_object.gatewayports {
            i.gatewayports = String::new();
        }
        if i.passwordauthentication == default_output_object.passwordauthentication {
            i.passwordauthentication = String::new();
        }
        if i.pubkeyauthentication == default_output_object.pubkeyauthentication {
            i.pubkeyauthentication = String::new();
        }
        if i.stricthostkeychecking == default_output_object.stricthostkeychecking {
            i.stricthostkeychecking = String::new();
        }
        if i.connecttimeout == default_output_object.connecttimeout {
            i.connecttimeout = String::new();
        }
        if i.controlmaster == default_output_object.controlmaster {
            i.controlmaster = String::new();
        }
        if i.controlpath == default_output_object.controlpath {
            i.controlpath = String::new();
        }
        if i.controlpersist == default_output_object.controlpersist {
            i.controlpersist = String::new();
        }
        if i.compression == default_output_object.compression {
            i.compression = String::new();
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
            options: format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
                c.options,
                c.identityfile,
                c.identitiesonly,
                c.clearallforwardings,
                c.exitonforwardfailure,
                c.forwardagent,
                c.forwardx11,
                c.forwardx11timeout,
                c.forwardx11trusted,
                c.serveralivecountmax,
                c.serveraliveinterval,
                c.gatewayports,
                c.passwordauthentication,
                c.pubkeyauthentication,
                c.stricthostkeychecking,
                c.connecttimeout,
                c.controlmaster,
                c.controlpath,
                c.controlpersist,
                c.compression,
            )
        };
        app.ssh_connections.push(import);
    }
}

fn get_includes(ssh_config_paths: &mut Vec<PathBuf>, config: BufReader<File>) {
    for line_result in config.lines() {
        match line_result {
            Ok(line) => {
                let homedir = env::home_dir().unwrap();
                let line = line.trim()
                .replace("=", " ")
                .replace("~", &homedir.display().to_string());
                if !line.is_empty()
                    && !line.starts_with('#')
                    && line.to_lowercase().starts_with("include ")
                {
                    let line = line.split_whitespace();
                    for part in line {
                        if part.to_lowercase() != "include" {
                            if Path::new(part).is_absolute() {
                                if part.contains("*") {
                                    for entry in glob(part).expect("Failed to read glob pattern") {
                                        match entry {
                                            Ok(path) => ssh_config_paths.push(PathBuf::from(path)),
                                            Err(e) => println!("{:?}", e),
                                        }
                                    }
                                }
                                else {ssh_config_paths.push(PathBuf::from(part))}
                            } else if !Path::new(part).is_absolute() {
                                let part = format!("{}/.ssh/{}", homedir.display(), part);
                                if part.contains("*") {
                                    for entry in glob(&part).expect("Failed to read glob pattern") {
                                        match entry {
                                            Ok(path) => ssh_config_paths.push(PathBuf::from(path)),
                                            Err(e) => println!("{:?}", e),
                                        }
                                    }
                                }
                                else {ssh_config_paths.push(PathBuf::from(part))}
                            }
                        }
                    }
                }
            }
            Err(text) => eprintln!("Error config reading: {}", text),
        }
    }
}