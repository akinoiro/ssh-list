use crate::*;
use glob::glob;
use std::fs::File;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(PartialEq, Clone)]
pub struct SSHConfigConnection {
    server_name: String,
    username: String,
    hostname: String,
    port: String,
    options: Vec<(String, String)>, 
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

    if check_systemsshconfig_path(PathBuf::from("/etc/ssh/ssh_config")) {
        ssh_config_paths.push(PathBuf::from("/etc/ssh/ssh_config"));
        let config = load_config(&ssh_config_paths.last().unwrap());
        get_includes(&mut ssh_config_paths, config);
    }

    if check_systemsshconfig_path(PathBuf::from("C:\\ProgramData\\ssh\\ssh_config")) {
        ssh_config_paths.push(PathBuf::from("C:\\ProgramData\\ssh\\ssh_config"));
        let config = load_config(&ssh_config_paths.last().unwrap());
        get_includes(&mut ssh_config_paths, config);
    }

    for ssh_config_path in ssh_config_paths {
        if !check_blank_sshconfig(&ssh_config_path) {
            let config = load_config(&ssh_config_path);
            get_names(&mut names, config);
        }
    }
    
    parse_from_ssh(names, &mut sshconfig);
    compare_with_defaults(&mut sshconfig, default_output_object);
    add_to_appconfig(sshconfig, app);
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

fn check_systemsshconfig_path(path:PathBuf) -> bool {
    match fs::exists(&path) {
        Ok(true)  => true,
        Ok(false)  => false,
        Err(_) => false
    }
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
                if !line.is_empty() && !line.starts_with('#') && line.to_lowercase().starts_with("host ") {
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
        let output_raw = Command::new("ssh").arg("-G").arg(name).output().unwrap();
        let output = std::str::from_utf8(&output_raw.stdout).unwrap();
        let error = std::str::from_utf8(&output_raw.stderr).unwrap();

        if !error.contains("Cannot fork") {
            let mut connection = SSHConfigConnection {
                server_name: name.to_string(),
                username: String::new(),
                hostname: String::new(),
                port: String::new(),
                options: vec![],
            };
            for line in output.lines() {
                if let Some((key,value)) = line.trim().split_once(' ') {
                    let key = key.to_lowercase();
                    let value = value.to_string();
                    match key.as_str() {
                        "host" => connection.server_name = value,
                        "user" => connection.username = value,
                        "hostname" => connection.hostname = value,
                        "port" => connection.port = value,
                        _ => connection.options.push((key, value))
                    }
                }
            }
            sshconfig.push(connection);
        } else {
            let output = Command::new("ssh").arg("-G").arg(name).arg("uptime").output().unwrap();
            let output = std::str::from_utf8(&output.stdout).unwrap();
            let mut connection = SSHConfigConnection {
                server_name: name.to_string(),
                username: String::new(),
                hostname: String::new(),
                port: String::new(),
                options: vec![],
            };
            for line in output.lines() {
                if let Some((key,value)) = line.trim().split_once(' ') {
                    let key = key.to_lowercase();
                    let value = value.to_string();
                    match key.as_str() {
                        "host" => connection.server_name = value,
                        "user" => connection.username = value,
                        "hostname" => connection.hostname = value,
                        "port" => connection.port = value,
                        _ => connection.options.push((key, value))
                    }
                }
            }
            sshconfig.push(connection);
        }
    }
}

fn compare_with_defaults(sshconfig: &mut Vec<SSHConfigConnection>, default_output_object: SSHConfigConnection) {
    for config in sshconfig {
        let mut new_options: Vec<(String, String)> = vec![];
        for option in &config.options {
            if !default_output_object.options.contains(&option) {
                new_options.push(option.clone());
            }
        }
        config.options = new_options;
    }
}

fn get_options(option: &str, value: &str) -> String {
    let options_hashmap = HashMap::from([
        ("addkeystoagent", "AddKeysToAgent"),
        ("addressfamily", "AddressFamily"),
        ("batchmode", "BatchMode"),
        ("bindaddress", "BindAddress"),
        ("bindinterface", "BindInterface"),
        ("canonicaldomains", "CanonicalDomains"),
        ("canonicalizefallbacklocal", "CanonicalizeFallbackLocal"),
        ("canonicalizehostname", "CanonicalizeHostname"),
        ("canonicalizemaxdots", "CanonicalizeMaxDots"),
        ("canonicalizepermittedcnames", "CanonicalizePermittedCNAMEs"),
        ("casignaturealgorithms", "CASignatureAlgorithms"),
        ("certificatefile", "CertificateFile"),
        ("channeltimeout", "ChannelTimeout"),
        ("checkhostip", "CheckHostIP"),
        ("ciphers", "Ciphers"),
        ("clearallforwardings", "ClearAllForwardings"),
        ("compression", "Compression"),
        ("connectionattempts", "ConnectionAttempts"),
        ("connecttimeout", "ConnectTimeout"),
        ("controlmaster", "ControlMaster"),
        ("controlpath", "ControlPath"),
        ("controlpersist", "ControlPersist"),
        ("enableescapecommandline", "EnableEscapeCommandline"),
        ("enablesshkeysign", "EnableSSHKeysign"),
        ("escapechar", "EscapeChar"),
        ("exitonforwardfailure", "ExitOnForwardFailure"),
        ("fingerprinthash", "FingerprintHash"),
        ("forkafterauthentication", "ForkAfterAuthentication"),
        ("forwardagent", "ForwardAgent"),
        ("forwardx11", "ForwardX11"),
        ("forwardx11timeout", "ForwardX11Timeout"),
        ("forwardx11trusted", "ForwardX11Trusted"),
        ("gatewayports", "GatewayPorts"),
        ("globalknownhostsfile", "GlobalKnownHostsFile"),
        ("gssapiauthentication", "GSSAPIAuthentication"),
        ("gssapidelegatecredentials", "GSSAPIDelegateCredentials"),
        ("hashknownhosts", "HashKnownHosts"),
        ("hostbasedacceptedalgorithms", "HostbasedAcceptedAlgorithms"),
        ("hostbasedauthentication", "HostbasedAuthentication"),
        ("hostkeyalgorithms", "HostKeyAlgorithms"),
        ("hostkeyalias", "HostKeyAlias"),
        ("identitiesonly", "IdentitiesOnly"),
        ("identityagent", "IdentityAgent"),
        ("ignoreunknown", "IgnoreUnknown"),
        ("ipqos", "IPQoS"),
        ("kbdinteractiveauthentication", "KbdInteractiveAuthentication"),
        ("kbdinteractivedevices", "KbdInteractiveDevices"),
        ("kexalgorithms", "KexAlgorithms"),
        ("knownhostscommand", "KnownHostsCommand"),
        ("loglevel", "LogLevel"),
        ("logverbose", "LogVerbose"),
        ("macs", "MACs"),
        ("nohostauthenticationforlocalhost", "NoHostAuthenticationForLocalhost"),
        ("numberofpasswordprompts", "NumberOfPasswordPrompts"),
        ("obscurekeystroketiming", "ObscureKeystrokeTiming"),
        ("passwordauthentication", "PasswordAuthentication"),
        ("permitlocalcommand", "PermitLocalCommand"),
        ("permitremoteopen", "PermitRemoteOpen"),
        ("pkcs11provider", "PKCS11Provider"),
        ("preferredauthentications", "PreferredAuthentications"),
        ("proxyusefdpass", "ProxyUseFdpass"),
        ("pubkeyacceptedalgorithms", "PubkeyAcceptedAlgorithms"),
        ("pubkeyauthentication", "PubkeyAuthentication"),
        ("refuseconnection", "RefuseConnection"),
        ("rekeylimit", "RekeyLimit"),
        ("requesttty", "RequestTTY"),
        ("requiredrsasize", "RequiredRSASize"),
        ("revokedhostkeys", "RevokedHostKeys"),
        ("securitykeyprovider", "SecurityKeyProvider"),
        ("sendenv", "SendEnv"),
        ("serveralivecountmax", "ServerAliveCountMax"),
        ("serveraliveinterval", "ServerAliveInterval"),
        ("sessiontype", "SessionType"),
        ("setenv", "SetEnv"),
        ("stdinnull", "StdinNull"),
        ("streamlocalbindmask", "StreamLocalBindMask"),
        ("streamlocalbindunlink", "StreamLocalBindUnlink"),
        ("stricthostkeychecking", "StrictHostKeyChecking"),
        ("syslogfacility", "SyslogFacility"),
        ("tag", "Tag"),
        ("tcpkeepalive", "TCPKeepAlive"),
        ("tunnel", "Tunnel"),
        ("tunneldevice", "TunnelDevice"),
        ("updatehostkeys", "UpdateHostKeys"),
        ("userknownhostsfile", "UserKnownHostsFile"),
        ("verifyhostkeydns", "VerifyHostKeyDNS"),
        ("versionaddendum", "VersionAddendum"),
        ("visualhostkey", "VisualHostKey"),
        ("warnweakcrypto", "WarnWeakCrypto"),
        ("xauthlocation", "XAuthLocation"),
    ]);

    match option {
        "localforward" => {
            if !value.contains("[socks]:0") {
                let mut line = value.split_whitespace();
                let port = line.next().unwrap_or_default();
                let address = line.next().unwrap_or_default();
                format!("-R {}:{} ", port, address)
            } else {
                let mut line = value.split_whitespace();
                let port = line.next().unwrap_or_default();
                format!("-R {} ", port)
            }
        },
        "remoteforward" => {
            let mut line = value.split_whitespace();
            let port = line.next().unwrap_or_default();
            let address = line.next().unwrap_or_default();
            format!("-L {}:{} ", port, address)
        },
        "dynamicforward" => format!("-D {} ", value),
        "identityfile" => format!("-i {} ", value),
        "localcommand" => format!("-o LocalCommand='{}' ", value),
        "proxycommand" => format!("-o ProxyCommand='{}' ", value),
        "proxyjump" => format!("-J {} ", value),
        "remotecommand" => format!("-o RemoteCommand='{}' ", value),
        _ => format!("-o {}={} ", options_hashmap.get(option).unwrap_or(&option), value)
    }
}

fn add_to_appconfig(sshconfig: Vec<SSHConfigConnection>, app: &mut App) {
    for connection in sshconfig {
        let mut all_options = String::new();
        for (key, value) in &connection.options {
            let option = get_options(key, value);
            all_options.push_str(&option);
        }
        let import = SSHConnection {
            server_name: connection.server_name,
            group_name: String::new(),
            username: connection.username,
            hostname: connection.hostname,
            port: connection.port,
            options: all_options,
        };
        app.ssh_connections.push(import);
    }
}

fn get_includes(ssh_config_paths: &mut Vec<PathBuf>, config: BufReader<File>) {
    for line_result in config.lines() {
        match line_result {
            Ok(line) => {
                let homedir = env::home_dir().unwrap();
                let line = line
                    .trim()
                    .replace("=", " ")
                    .replace("~", &homedir.display().to_string());
                if !line.is_empty() && !line.starts_with('#') && line.to_lowercase().starts_with("include ") {
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
                                } else {
                                    ssh_config_paths.push(PathBuf::from(part))
                                }
                            } else if !Path::new(part).is_absolute() {
                                let part = format!("{}/.ssh/{}", homedir.display(), part);
                                if part.contains("*") {
                                    for entry in glob(&part).expect("Failed to read glob pattern") {
                                        match entry {
                                            Ok(path) => ssh_config_paths.push(PathBuf::from(path)),
                                            Err(e) => println!("{:?}", e),
                                        }
                                    }
                                } else {
                                    ssh_config_paths.push(PathBuf::from(part))
                                }
                            }
                        }
                    }
                }
            }
            Err(text) => eprintln!("Error config reading: {}", text),
        }
    }
}
