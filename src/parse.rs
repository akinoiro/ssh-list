use crate::*;
use glob::glob;
use std::fs::File;
use std::io::{BufRead, BufReader};
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
    addkeystoagent: String,
    addressfamily: String,
    batchmode: String,
    bindaddress: String,
    bindinterface: String,
    canonicaldomains: String,
    canonicalizefallbacklocal: String,
    canonicalizehostname: String,
    canonicalizemaxdots: String,
    canonicalizepermittedcnames: String,
    casignaturealgorithms: String,
    certificatefile: String,
    channeltimeout: String,
    checkhostip: String,
    ciphers: String,
    connectionattempts: String,
    enableescapecommandline: String,
    enablesshkeysign: String,
    escapechar: String,
    fingerprinthash: String,
    globalknownhostsfile: String,
    gssapiauthentication: String,
    gssapidelegatecredentials: String,
    hashknownhosts: String,
    hostbasedacceptedalgorithms: String,
    hostbasedauthentication: String,
    hostkeyalgorithms: String,
    hostkeyalias: String,
    identityagent: String,
    ignoreunknown: String,
    ipqos: String,
    kbdinteractiveauthentication: String,
    kbdinteractivedevices: String,
    kexalgorithms: String,
    knownhostscommand: String,
    localcommand: String,
    loglevel: String,
    logverbose: String,
    macs: String,
    nohostauthenticationforlocalhost: String,
    numberofpasswordprompts: String,
    obscurekeystroketiming: String,
    permitlocalcommand: String,
    permitremoteopen: String,
    pkcs11provider: String,
    preferredauthentications: String,
    proxycommand: String,
    proxyusefdpass: String,
    pubkeyacceptedalgorithms: String,
    refuseconnection: String,
    rekeylimit: String,
    remotecommand: String,
    requesttty: String,
    requiredrsasize: String,
    revokedhostkeys: String,
    securitykeyprovider: String,
    sendenv: String,
    sessiontype: String,
    setenv: String,
    stdinnull: String,
    streamlocalbindunlink: String,
    tag: String,
    tcpkeepalive: String,
    tunnel: String,
    tunneldevice: String,
    updatehostkeys: String,
    userknownhostsfile: String,
    versionaddendum: String,
    verifyhostkeydns: String,
    visualhostkey: String,
    warnweakcrypto: String,
    xauthlocation: String,
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
        let output = Command::new("ssh").arg("-G").arg(name).output().expect("Error");
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
            addkeystoagent: String::new(),
            addressfamily: String::new(),
            batchmode: String::new(),
            bindaddress: String::new(),
            bindinterface: String::new(),
            canonicaldomains: String::new(),
            canonicalizefallbacklocal: String::new(),
            canonicalizehostname: String::new(),
            canonicalizemaxdots: String::new(),
            canonicalizepermittedcnames: String::new(),
            casignaturealgorithms: String::new(),
            certificatefile: String::new(),
            channeltimeout: String::new(),
            checkhostip: String::new(),
            ciphers: String::new(),
            connectionattempts: String::new(),
            enableescapecommandline: String::new(),
            enablesshkeysign: String::new(),
            escapechar: String::new(),
            fingerprinthash: String::new(),
            globalknownhostsfile: String::new(),
            gssapiauthentication: String::new(),
            gssapidelegatecredentials: String::new(),
            hashknownhosts: String::new(),
            hostbasedacceptedalgorithms: String::new(),
            hostbasedauthentication: String::new(),
            hostkeyalgorithms: String::new(),
            hostkeyalias: String::new(),
            identityagent: String::new(),
            ignoreunknown: String::new(),
            ipqos: String::new(),
            kbdinteractiveauthentication: String::new(),
            kbdinteractivedevices: String::new(),
            kexalgorithms: String::new(),
            knownhostscommand: String::new(),
            localcommand: String::new(),
            loglevel: String::new(),
            logverbose: String::new(),
            macs: String::new(),
            nohostauthenticationforlocalhost: String::new(),
            numberofpasswordprompts: String::new(),
            obscurekeystroketiming: String::new(),
            permitlocalcommand: String::new(),
            permitremoteopen: String::new(),
            pkcs11provider: String::new(),
            preferredauthentications: String::new(),
            proxycommand: String::new(),
            proxyusefdpass: String::new(),
            pubkeyacceptedalgorithms: String::new(),
            refuseconnection: String::new(),
            rekeylimit: String::new(),
            remotecommand: String::new(),
            requesttty: String::new(),
            requiredrsasize: String::new(),
            revokedhostkeys: String::new(),
            securitykeyprovider: String::new(),
            sendenv: String::new(),
            sessiontype: String::new(),
            setenv: String::new(),
            stdinnull: String::new(),
            streamlocalbindunlink: String::new(),
            tag: String::new(),
            tcpkeepalive: String::new(),
            tunnel: String::new(),
            tunneldevice: String::new(),
            updatehostkeys: String::new(),
            userknownhostsfile: String::new(),
            versionaddendum: String::new(),
            verifyhostkeydns: String::new(),
            visualhostkey: String::new(),
            warnweakcrypto: String::new(),
            xauthlocation: String::new(),
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
                connection
                    .options
                    .push_str(format!("-L {}:{} ", port, address).as_str());
            }
            if line.starts_with("remoteforward ") {
                if !line.contains("[socks]:0") {
                    let mut line = line.split_whitespace();
                    line.next();
                    let port = line.next().unwrap_or_default();
                    let address = line.next().unwrap_or_default();
                    connection
                        .options
                        .push_str(format!("-R {}:{} ", port, address).as_str());
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
            if line.starts_with("addkeystoagent ") {
                let arg = line.replace("addkeystoagent ", "-o AddKeysToAgent=");
                connection.addkeystoagent.push_str(&arg);
                connection.addkeystoagent.push(' ');
            }
            if line.starts_with("addressfamily ") {
                let arg = line.replace("addressfamily ", "-o AddressFamily=");
                connection.addressfamily.push_str(&arg);
                connection.addressfamily.push(' ');
            }
            if line.starts_with("batchmode ") {
                let arg = line.replace("batchmode ", "-o BatchMode=");
                connection.batchmode.push_str(&arg);
                connection.batchmode.push(' ');
            }
            if line.starts_with("bindaddress ") {
                let arg = line.replace("bindaddress ", "-o BindAddress=");
                connection.bindaddress.push_str(&arg);
                connection.bindaddress.push(' ');
            }
            if line.starts_with("bindinterface ") {
                let arg = line.replace("bindinterface ", "-o BindInterface=");
                connection.bindinterface.push_str(&arg);
                connection.bindinterface.push(' ');
            }
            if line.starts_with("canonicaldomains ") {
                let arg = line.replace("canonicaldomains ", "-o CanonicalDomains=");
                connection.canonicaldomains.push_str(&arg);
                connection.canonicaldomains.push(' ');
            }
            if line.starts_with("canonicalizefallbacklocal ") {
                let arg = line.replace("canonicalizefallbacklocal ", "-o CanonicalizeFallbackLocal=");
                connection.canonicalizefallbacklocal.push_str(&arg);
                connection.canonicalizefallbacklocal.push(' ');
            }
            if line.starts_with("canonicalizehostname ") {
                let arg = line.replace("canonicalizehostname ", "-o CanonicalizeHostname=");
                connection.canonicalizehostname.push_str(&arg);
                connection.canonicalizehostname.push(' ');
            }
            if line.starts_with("canonicalizemaxdots ") {
                let arg = line.replace("canonicalizemaxdots ", "-o CanonicalizeMaxDots=");
                connection.canonicalizemaxdots.push_str(&arg);
                connection.canonicalizemaxdots.push(' ');
            }
            if line.to_lowercase().starts_with("canonicalizepermittedcnames ") {
                let arg = line.to_lowercase().replace("canonicalizepermittedcnames ", "-o CanonicalizePermittedCNAMEs=");
                connection.canonicalizepermittedcnames.push_str(&arg);
                connection.canonicalizepermittedcnames.push(' ');
            }
            if line.starts_with("casignaturealgorithms ") {
                let arg = line.replace("casignaturealgorithms ", "-o CASignatureAlgorithms=");
                connection.casignaturealgorithms.push_str(&arg);
                connection.casignaturealgorithms.push(' ');
            }
            if line.starts_with("certificatefile ") {
                let arg = line.replace("certificatefile ", "-o CertificateFile=");
                connection.certificatefile.push_str(&arg);
                connection.certificatefile.push(' ');
            }
            if line.starts_with("channeltimeout ") {
                let arg = line.replace("channeltimeout ", "-o ChannelTimeout=");
                connection.channeltimeout.push_str(&arg);
                connection.channeltimeout.push(' ');
            }
            if line.starts_with("checkhostip ") {
                let arg = line.replace("checkhostip ", "-o CheckHostIP=");
                connection.checkhostip.push_str(&arg);
                connection.checkhostip.push(' ');
            }
            if line.starts_with("ciphers ") {
                let arg = line.replace("ciphers ", "-o Ciphers=");
                connection.ciphers.push_str(&arg);
                connection.ciphers.push(' ');
            }
            if line.starts_with("connectionattempts ") {
                let arg = line.replace("connectionattempts ", "-o ConnectionAttempts=");
                connection.connectionattempts.push_str(&arg);
                connection.connectionattempts.push(' ');
            }
            if line.starts_with("enableescapecommandline ") {
                let arg = line.replace("enableescapecommandline ", "-o EnableEscapeCommandline=");
                connection.enableescapecommandline.push_str(&arg);
                connection.enableescapecommandline.push(' ');
            }
            if line.starts_with("enablesshkeysign ") {
                let arg = line.replace("enablesshkeysign ", "-o EnableSSHKeysign=");
                connection.enablesshkeysign.push_str(&arg);
                connection.enablesshkeysign.push(' ');
            }
            if line.starts_with("escapechar ") {
                let arg = line.replace("escapechar ", "-o EscapeChar=");
                connection.escapechar.push_str(&arg);
                connection.escapechar.push(' ');
            }
            if line.starts_with("fingerprinthash ") {
                let arg = line.replace("fingerprinthash ", "-o FingerprintHash=");
                connection.fingerprinthash.push_str(&arg);
                connection.fingerprinthash.push(' ');
            }
            if line.starts_with("globalknownhostsfile ") {
                let arg = line.replace("globalknownhostsfile ", "-o GlobalKnownHostsFile=");
                connection.globalknownhostsfile.push_str(&arg);
                connection.globalknownhostsfile.push(' ');
            }
            if line.starts_with("gssapiauthentication ") {
                let arg = line.replace("gssapiauthentication ", "-o GSSAPIAuthentication=");
                connection.gssapiauthentication.push_str(&arg);
                connection.gssapiauthentication.push(' ');
            }
            if line.starts_with("gssapidelegatecredentials ") {
                let arg = line.replace("gssapidelegatecredentials ", "-o GSSAPIDelegateCredentials=");
                connection.gssapidelegatecredentials.push_str(&arg);
                connection.gssapidelegatecredentials.push(' ');
            }
            if line.starts_with("hashknownhosts ") {
                let arg = line.replace("hashknownhosts ", "-o HashKnownHosts=");
                connection.hashknownhosts.push_str(&arg);
                connection.hashknownhosts.push(' ');
            }
            if line.starts_with("hostbasedacceptedalgorithms ") {
                let arg = line.replace("hostbasedacceptedalgorithms ", "-o HostbasedAcceptedAlgorithms=");
                connection.hostbasedacceptedalgorithms.push_str(&arg);
                connection.hostbasedacceptedalgorithms.push(' ');
            }
            if line.starts_with("hostbasedauthentication ") {
                let arg = line.replace("hostbasedauthentication ", "-o HostbasedAuthentication=");
                connection.hostbasedauthentication.push_str(&arg);
                connection.hostbasedauthentication.push(' ');
            }
            if line.starts_with("hostkeyalgorithms ") {
                let arg = line.replace("hostkeyalgorithms ", "-o HostKeyAlgorithms=");
                connection.hostkeyalgorithms.push_str(&arg);
                connection.hostkeyalgorithms.push(' ');
            }
            if line.starts_with("hostkeyalias ") {
                let arg = line.replace("hostkeyalias ", "-o HostKeyAlias=");
                connection.hostkeyalias.push_str(&arg);
                connection.hostkeyalias.push(' ');
            }
            if line.starts_with("identityagent ") {
                let arg = line.replace("identityagent ", "-o IdentityAgent=");
                connection.identityagent.push_str(&arg);
                connection.identityagent.push(' ');
            }
            if line.starts_with("ignoreunknown ") {
                let arg = line.replace("ignoreunknown ", "-o IgnoreUnknown=");
                connection.ignoreunknown.push_str(&arg);
                connection.ignoreunknown.push(' ');
            }
            if line.starts_with("ipqos ") {
                let arg = line.replace("ipqos ", "-o IPQoS=");
                connection.ipqos.push_str(&arg);
                connection.ipqos.push(' ');
            }
            if line.starts_with("kbdinteractiveauthentication ") {
                let arg = line.replace("kbdinteractiveauthentication ", "-o KbdInteractiveAuthentication=");
                connection.kbdinteractiveauthentication.push_str(&arg);
                connection.kbdinteractiveauthentication.push(' ');
            }
            if line.starts_with("kbdinteractivedevices ") {
                let arg = line.replace("kbdinteractivedevices ", "-o KbdInteractiveDevices=");
                connection.kbdinteractivedevices.push_str(&arg);
                connection.kbdinteractivedevices.push(' ');
            }
            if line.starts_with("kexalgorithms ") {
                let arg = line.replace("kexalgorithms ", "-o KexAlgorithms=");
                connection.kexalgorithms.push_str(&arg);
                connection.kexalgorithms.push(' ');
            }
            if line.starts_with("knownhostscommand ") {
                let arg = line.replace("knownhostscommand ", "-o KnownHostsCommand=");
                connection.knownhostscommand.push_str(&arg);
                connection.knownhostscommand.push(' ');
            }
            if line.starts_with("localcommand ") {
                let arg = line.replace("localcommand ", "-o LocalCommand=");
                connection.localcommand.push_str(&arg);
                connection.localcommand.push(' ');
            }
            if line.starts_with("loglevel ") {
                let arg = line.replace("loglevel ", "-o LogLevel=");
                connection.loglevel.push_str(&arg);
                connection.loglevel.push(' ');
            }
            if line.starts_with("logverbose ") {
                let arg = line.replace("logverbose ", "-o LogVerbose=");
                connection.logverbose.push_str(&arg);
                connection.logverbose.push(' ');
            }
            if line.starts_with("macs ") {
                let arg = line.replace("macs ", "-o MACs=");
                connection.macs.push_str(&arg);
                connection.macs.push(' ');
            }
            if line.starts_with("nohostauthenticationforlocalhost ") {
                let arg = line.replace(
                    "nohostauthenticationforlocalhost ",
                    "-o NoHostAuthenticationForLocalhost=",
                );
                connection.nohostauthenticationforlocalhost.push_str(&arg);
                connection.nohostauthenticationforlocalhost.push(' ');
            }
            if line.starts_with("numberofpasswordprompts ") {
                let arg = line.replace("numberofpasswordprompts ", "-o NumberOfPasswordPrompts=");
                connection.numberofpasswordprompts.push_str(&arg);
                connection.numberofpasswordprompts.push(' ');
            }
            if line.starts_with("obscurekeystroketiming ") {
                let arg = line.replace("obscurekeystroketiming ", "-o ObscureKeystrokeTiming=");
                connection.obscurekeystroketiming.push_str(&arg);
                connection.obscurekeystroketiming.push(' ');
            }
            if line.starts_with("permitlocalcommand ") {
                let arg = line.replace("permitlocalcommand ", "-o PermitLocalCommand=");
                connection.permitlocalcommand.push_str(&arg);
                connection.permitlocalcommand.push(' ');
            }
            if line.starts_with("permitremoteopen ") {
                let arg = line.replace("permitremoteopen ", "-o PermitRemoteOpen=");
                connection.permitremoteopen.push_str(&arg);
                connection.permitremoteopen.push(' ');
            }
            if line.starts_with("pkcs11provider ") {
                let arg = line.replace("pkcs11provider ", "-o PKCS11Provider=");
                connection.pkcs11provider.push_str(&arg);
                connection.pkcs11provider.push(' ');
            }
            if line.starts_with("preferredauthentications ") {
                let arg = line.replace("preferredauthentications ", "-o PreferredAuthentications=");
                connection.preferredauthentications.push_str(&arg);
                connection.preferredauthentications.push(' ');
            }
            if line.starts_with("proxycommand ") {
                let arg = line.replace("proxycommand ", "-o ProxyCommand=");
                connection.proxycommand.push_str(&arg);
                connection.proxycommand.push(' ');
            }
            if line.starts_with("proxyusefdpass ") {
                let arg = line.replace("proxyusefdpass ", "-o ProxyUseFdpass=");
                connection.proxyusefdpass.push_str(&arg);
                connection.proxyusefdpass.push(' ');
            }
            if line.starts_with("pubkeyacceptedalgorithms ") {
                let arg = line.replace("pubkeyacceptedalgorithms ", "-o PubkeyAcceptedAlgorithms=");
                connection.pubkeyacceptedalgorithms.push_str(&arg);
                connection.pubkeyacceptedalgorithms.push(' ');
            }
            if line.starts_with("refuseconnection ") {
                let arg = line.replace("refuseconnection ", "-o RefuseConnection=");
                connection.refuseconnection.push_str(&arg);
                connection.refuseconnection.push(' ');
            }
            if line.starts_with("rekeylimit ") {
                let arg = line.replace("rekeylimit ", "-o RekeyLimit=");
                connection.rekeylimit.push_str(&arg);
                connection.rekeylimit.push(' ');
            }
            if line.starts_with("remotecommand ") {
                let arg = line.replace("remotecommand ", "-o RemoteCommand=");
                connection.remotecommand.push_str(&arg);
                connection.remotecommand.push(' ');
            }
            if line.starts_with("requesttty ") {
                let arg = line.replace("requesttty ", "-o RequestTTY=");
                connection.requesttty.push_str(&arg);
                connection.requesttty.push(' ');
            }
            if line.starts_with("requiredrsasize ") {
                let arg = line.replace("requiredrsasize ", "-o RequiredRSASize=");
                connection.requiredrsasize.push_str(&arg);
                connection.requiredrsasize.push(' ');
            }
            if line.starts_with("revokedhostkeys ") {
                let arg = line.replace("revokedhostkeys ", "-o RevokedHostKeys=");
                connection.revokedhostkeys.push_str(&arg);
                connection.revokedhostkeys.push(' ');
            }
            if line.starts_with("securitykeyprovider ") {
                let arg = line.replace("securitykeyprovider ", "-o SecurityKeyProvider=");
                connection.securitykeyprovider.push_str(&arg);
                connection.securitykeyprovider.push(' ');
            }
            if line.starts_with("sendenv ") {
                let arg = line.replace("sendenv ", "-o SendEnv=");
                connection.sendenv.push_str(&arg);
                connection.sendenv.push(' ');
            }
            if line.starts_with("sessiontype ") {
                let arg = line.replace("sessiontype ", "-o SessionType=");
                connection.sessiontype.push_str(&arg);
                connection.sessiontype.push(' ');
            }
            if line.starts_with("setenv ") {
                let arg = line.replace("setenv ", "-o SetEnv=");
                connection.setenv.push_str(&arg);
                connection.setenv.push(' ');
            }
            if line.starts_with("stdinnull ") {
                let arg = line.replace("stdinnull ", "-o StdinNull=");
                connection.stdinnull.push_str(&arg);
                connection.stdinnull.push(' ');
            }
            if line.starts_with("streamlocalbindunlink ") {
                let arg = line.replace("streamlocalbindunlink ", "-o StreamLocalBindUnlink=");
                connection.streamlocalbindunlink.push_str(&arg);
                connection.streamlocalbindunlink.push(' ');
            }
            if line.starts_with("tag ") {
                let arg = line.replace("tag ", "-o Tag=");
                connection.tag.push_str(&arg);
                connection.tag.push(' ');
            }
            if line.starts_with("tcpkeepalive ") {
                let arg = line.replace("tcpkeepalive ", "-o TCPKeepAlive=");
                connection.tcpkeepalive.push_str(&arg);
                connection.tcpkeepalive.push(' ');
            }
            if line.starts_with("tunnel ") {
                let arg = line.replace("tunnel ", "-o Tunnel=");
                connection.tunnel.push_str(&arg);
                connection.tunnel.push(' ');
            }
            if line.starts_with("tunneldevice ") {
                let arg = line.replace("tunneldevice ", "-o TunnelDevice=");
                connection.tunneldevice.push_str(&arg);
                connection.tunneldevice.push(' ');
            }
            if line.starts_with("updatehostkeys ") {
                let arg = line.replace("updatehostkeys ", "-o UpdateHostKeys=");
                connection.updatehostkeys.push_str(&arg);
                connection.updatehostkeys.push(' ');
            }
            if line.starts_with("userknownhostsfile ") {
                let arg = line.replace("userknownhostsfile ", "-o UserKnownHostsFile=");
                connection.userknownhostsfile.push_str(&arg);
                connection.userknownhostsfile.push(' ');
            }
            if line.starts_with("versionaddendum ") {
                let arg = line.replace("versionaddendum ", "-o VersionAddendum=");
                connection.versionaddendum.push_str(&arg);
                connection.versionaddendum.push(' ');
            }
            if line.starts_with("verifyhostkeydns ") {
                let arg = line.replace("verifyhostkeydns ", "-o VerifyHostKeyDNS=");
                connection.verifyhostkeydns.push_str(&arg);
                connection.verifyhostkeydns.push(' ');
            }
            if line.starts_with("visualhostkey ") {
                let arg = line.replace("visualhostkey ", "-o VisualHostKey=");
                connection.visualhostkey.push_str(&arg);
                connection.visualhostkey.push(' ');
            }
            if line.starts_with("warnweakcrypto ") {
                let arg = line.replace("warnweakcrypto ", "-o WarnWeakCrypto=");
                connection.warnweakcrypto.push_str(&arg);
                connection.warnweakcrypto.push(' ');
            }
            if line.starts_with("xauthlocation ") {
                let arg = line.replace("xauthlocation ", "-o XAuthLocation=");
                connection.xauthlocation.push_str(&arg);
                connection.xauthlocation.push(' ');
            }
        }
        sshconfig.push(connection);
    }
}

fn compare_with_defaults(sshconfig: &mut Vec<SSHConfigConnection>, default_output_object: SSHConfigConnection) {
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
        // New options start here
        if i.addkeystoagent == default_output_object.addkeystoagent {
            i.addkeystoagent = String::new();
        }
        if i.addressfamily == default_output_object.addressfamily {
            i.addressfamily = String::new();
        }
        if i.batchmode == default_output_object.batchmode {
            i.batchmode = String::new();
        }
        if i.bindaddress == default_output_object.bindaddress {
            i.bindaddress = String::new();
        }
        if i.bindinterface == default_output_object.bindinterface {
            i.bindinterface = String::new();
        }
        if i.canonicaldomains == default_output_object.canonicaldomains {
            i.canonicaldomains = String::new();
        }
        if i.canonicalizefallbacklocal == default_output_object.canonicalizefallbacklocal {
            i.canonicalizefallbacklocal = String::new();
        }
        if i.canonicalizehostname == default_output_object.canonicalizehostname {
            i.canonicalizehostname = String::new();
        }
        if i.canonicalizemaxdots == default_output_object.canonicalizemaxdots {
            i.canonicalizemaxdots = String::new();
        }
        if i.canonicalizepermittedcnames == default_output_object.canonicalizepermittedcnames {
            i.canonicalizepermittedcnames = String::new();
        }
        if i.casignaturealgorithms == default_output_object.casignaturealgorithms {
            i.casignaturealgorithms = String::new();
        }
        if i.certificatefile == default_output_object.certificatefile {
            i.certificatefile = String::new();
        }
        if i.channeltimeout == default_output_object.channeltimeout {
            i.channeltimeout = String::new();
        }
        if i.checkhostip == default_output_object.checkhostip {
            i.checkhostip = String::new();
        }
        if i.ciphers == default_output_object.ciphers {
            i.ciphers = String::new();
        }
        if i.connectionattempts == default_output_object.connectionattempts {
            i.connectionattempts = String::new();
        }
        if i.enableescapecommandline == default_output_object.enableescapecommandline {
            i.enableescapecommandline = String::new();
        }
        if i.enablesshkeysign == default_output_object.enablesshkeysign {
            i.enablesshkeysign = String::new();
        }
        if i.escapechar == default_output_object.escapechar {
            i.escapechar = String::new();
        }
        if i.fingerprinthash == default_output_object.fingerprinthash {
            i.fingerprinthash = String::new();
        }
        if i.globalknownhostsfile == default_output_object.globalknownhostsfile {
            i.globalknownhostsfile = String::new();
        }
        if i.gssapiauthentication == default_output_object.gssapiauthentication {
            i.gssapiauthentication = String::new();
        }
        if i.gssapidelegatecredentials == default_output_object.gssapidelegatecredentials {
            i.gssapidelegatecredentials = String::new();
        }
        if i.hashknownhosts == default_output_object.hashknownhosts {
            i.hashknownhosts = String::new();
        }
        if i.hostbasedacceptedalgorithms == default_output_object.hostbasedacceptedalgorithms {
            i.hostbasedacceptedalgorithms = String::new();
        }
        if i.hostbasedauthentication == default_output_object.hostbasedauthentication {
            i.hostbasedauthentication = String::new();
        }
        if i.hostkeyalgorithms == default_output_object.hostkeyalgorithms {
            i.hostkeyalgorithms = String::new();
        }
        if i.hostkeyalias == default_output_object.hostkeyalias {
            i.hostkeyalias = String::new();
        }
        if i.identityagent == default_output_object.identityagent {
            i.identityagent = String::new();
        }
        if i.ignoreunknown == default_output_object.ignoreunknown {
            i.ignoreunknown = String::new();
        }
        if i.ipqos == default_output_object.ipqos {
            i.ipqos = String::new();
        }
        if i.kbdinteractiveauthentication == default_output_object.kbdinteractiveauthentication {
            i.kbdinteractiveauthentication = String::new();
        }
        if i.kbdinteractivedevices == default_output_object.kbdinteractivedevices {
            i.kbdinteractivedevices = String::new();
        }
        if i.kexalgorithms == default_output_object.kexalgorithms {
            i.kexalgorithms = String::new();
        }
        if i.knownhostscommand == default_output_object.knownhostscommand {
            i.knownhostscommand = String::new();
        }
        if i.localcommand == default_output_object.localcommand {
            i.localcommand = String::new();
        }
        if i.loglevel == default_output_object.loglevel {
            i.loglevel = String::new();
        }
        if i.logverbose == default_output_object.logverbose {
            i.logverbose = String::new();
        }
        if i.macs == default_output_object.macs {
            i.macs = String::new();
        }
        if i.nohostauthenticationforlocalhost == default_output_object.nohostauthenticationforlocalhost {
            i.nohostauthenticationforlocalhost = String::new();
        }
        if i.numberofpasswordprompts == default_output_object.numberofpasswordprompts {
            i.numberofpasswordprompts = String::new();
        }
        if i.obscurekeystroketiming == default_output_object.obscurekeystroketiming {
            i.obscurekeystroketiming = String::new();
        }
        if i.permitlocalcommand == default_output_object.permitlocalcommand {
            i.permitlocalcommand = String::new();
        }
        if i.permitremoteopen == default_output_object.permitremoteopen {
            i.permitremoteopen = String::new();
        }
        if i.pkcs11provider == default_output_object.pkcs11provider {
            i.pkcs11provider = String::new();
        }
        if i.preferredauthentications == default_output_object.preferredauthentications {
            i.preferredauthentications = String::new();
        }
        if i.proxycommand == default_output_object.proxycommand {
            i.proxycommand = String::new();
        }
        if i.proxyusefdpass == default_output_object.proxyusefdpass {
            i.proxyusefdpass = String::new();
        }
        if i.pubkeyacceptedalgorithms == default_output_object.pubkeyacceptedalgorithms {
            i.pubkeyacceptedalgorithms = String::new();
        }
        if i.refuseconnection == default_output_object.refuseconnection {
            i.refuseconnection = String::new();
        }
        if i.rekeylimit == default_output_object.rekeylimit {
            i.rekeylimit = String::new();
        }
        if i.remotecommand == default_output_object.remotecommand {
            i.remotecommand = String::new();
        }
        if i.requesttty == default_output_object.requesttty {
            i.requesttty = String::new();
        }
        if i.requiredrsasize == default_output_object.requiredrsasize {
            i.requiredrsasize = String::new();
        }
        if i.revokedhostkeys == default_output_object.revokedhostkeys {
            i.revokedhostkeys = String::new();
        }
        if i.securitykeyprovider == default_output_object.securitykeyprovider {
            i.securitykeyprovider = String::new();
        }
        if i.sendenv == default_output_object.sendenv {
            i.sendenv = String::new();
        }
        if i.sessiontype == default_output_object.sessiontype {
            i.sessiontype = String::new();
        }
        if i.setenv == default_output_object.setenv {
            i.setenv = String::new();
        }
        if i.stdinnull == default_output_object.stdinnull {
            i.stdinnull = String::new();
        }
        if i.streamlocalbindunlink == default_output_object.streamlocalbindunlink {
            i.streamlocalbindunlink = String::new();
        }
        if i.tag == default_output_object.tag {
            i.tag = String::new();
        }
        if i.tcpkeepalive == default_output_object.tcpkeepalive {
            i.tcpkeepalive = String::new();
        }
        if i.tunnel == default_output_object.tunnel {
            i.tunnel = String::new();
        }
        if i.tunneldevice == default_output_object.tunneldevice {
            i.tunneldevice = String::new();
        }
        if i.updatehostkeys == default_output_object.updatehostkeys {
            i.updatehostkeys = String::new();
        }
        if i.userknownhostsfile == default_output_object.userknownhostsfile {
            i.userknownhostsfile = String::new();
        }
        if i.versionaddendum == default_output_object.versionaddendum {
            i.versionaddendum = String::new();
        }
        if i.verifyhostkeydns == default_output_object.verifyhostkeydns {
            i.verifyhostkeydns = String::new();
        }
        if i.visualhostkey == default_output_object.visualhostkey {
            i.visualhostkey = String::new();
        }
        if i.warnweakcrypto == default_output_object.warnweakcrypto {
            i.warnweakcrypto = String::new();
        }
        if i.xauthlocation == default_output_object.xauthlocation {
            i.xauthlocation = String::new();
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
            options: format!(
                "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
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
                c.addkeystoagent,
                c.addressfamily,
                c.batchmode,
                c.bindaddress,
                c.bindinterface,
                c.canonicaldomains,
                c.canonicalizefallbacklocal,
                c.canonicalizehostname,
                c.canonicalizemaxdots,
                c.canonicalizepermittedcnames,
                c.casignaturealgorithms,
                c.certificatefile,
                c.channeltimeout,
                c.checkhostip,
                c.ciphers,
                c.connectionattempts,
                c.enableescapecommandline,
                c.enablesshkeysign,
                c.escapechar,
                c.fingerprinthash,
                c.globalknownhostsfile,
                c.gssapiauthentication,
                c.gssapidelegatecredentials,
                c.hashknownhosts,
                c.hostbasedacceptedalgorithms,
                c.hostbasedauthentication,
                c.hostkeyalgorithms,
                c.hostkeyalias,
                c.identityagent,
                c.ignoreunknown,
                c.ipqos,
                c.kbdinteractiveauthentication,
                c.kbdinteractivedevices,
                c.kexalgorithms,
                c.knownhostscommand,
                c.localcommand,
                c.loglevel,
                c.logverbose,
                c.macs,
                c.nohostauthenticationforlocalhost,
                c.numberofpasswordprompts,
                c.obscurekeystroketiming,
                c.permitlocalcommand,
                c.permitremoteopen,
                c.pkcs11provider,
                c.preferredauthentications,
                c.proxycommand,
                c.proxyusefdpass,
                c.pubkeyacceptedalgorithms,
                c.refuseconnection,
                c.rekeylimit,
                c.remotecommand,
                c.requesttty,
                c.requiredrsasize,
                c.revokedhostkeys,
                c.securitykeyprovider,
                c.sendenv,
                c.sessiontype,
                c.setenv,
                c.stdinnull,
                c.streamlocalbindunlink,
                c.tag,
                c.tcpkeepalive,
                c.tunnel,
                c.tunneldevice,
                c.updatehostkeys,
                c.userknownhostsfile,
                c.versionaddendum,
                c.verifyhostkeydns,
                c.visualhostkey,
                c.warnweakcrypto,
                c.xauthlocation,
            ),
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
