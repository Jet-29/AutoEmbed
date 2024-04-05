use std::process::Command;

const HOST: &str = "Arachnid";
const WAIT_TIME: u64 = 2;

fn main() -> Result<(), &'static str> {
    let args = std::env::args();
    if args.len() > 1 {
        // TODO Added features for running other probe-rs commands
        return Err("Extra args are not supported yet");
    }

    let full_path = std::env::current_dir().unwrap();
    let current_dir = full_path.iter().last().unwrap().to_str().unwrap();

    let config =
        std::fs::read_to_string(".cargo/config.toml").or(Err("No cargo config file found"))?;

    let parsed_config = config
        .parse::<toml::Table>()
        .or(Err("Unable to parse cargo config"))?;

    let build_config = parsed_config
        .get("build")
        .ok_or("No build section in config")?;
    let target = build_config
        .get("target")
        .ok_or("No target specification in config")?
        .as_str()
        .ok_or("Target is not a string")?;

    let chip = from_target_to_chip(target)?;

    // Check Arachnid is online
    if !ping_host() {
        println!("Host not found attempting to start");
        attempt_to_turn_on_host(10)?;
    }

    // Begin commands
    // rsync
    Command::new("rsync")
        .args(["-a", "--delete"])
        .arg(format!("./../{current_dir}"))
        .arg(format!("{HOST}:embedded/"))
        .spawn()
        .or(Err("Failed to run rsync command"))?
        .wait()
        .or(Err("Failed to wait for rsync"))?;

    // cargo-embed
    Command::new("ssh")
        .args(["-t", HOST])
        .arg(format!(
            ". $HOME/.cargo/env; cd embedded/{current_dir}; cargo embed --chip {chip}"
        ))
        .spawn()
        .or(Err("Failed to run embed command"))?
        .wait()
        .or(Err("Failed to wait for ssh connection"))?;

    Ok(())
}

fn from_target_to_chip(target: &str) -> Result<&'static str, &'static str> {
    match target {
        "thumbv6m-none-eabi" => Ok("nRF51822_xxAA"),
        _ => Err("Target not recognised as a specific chip"),
    }
}

fn attempt_to_turn_on_host(attempts: u32) -> Result<(), &'static str> {
    if !Command::new("/mnt/c/Program Files (x86)/VMware/VMware Workstation/vmrun.exe")
        .args([
            "start",
            "c:\\Virtual Machines\\Arachnid\\Ubuntu.vmx",
            "noGui",
        ])
        .output()
        .unwrap()
        .status
        .success()
    {
        return Err("Failed to launch VM");
    }

    for attempt in 0..attempts {
        std::thread::sleep(std::time::Duration::from_secs(1));
        if ping_host() {
            println!("Ping attempt: {attempt} [ SUCCESS ]");
            println!("Waiting {WAIT_TIME} seconds for system to fully start");
            std::thread::sleep(std::time::Duration::from_secs(WAIT_TIME));
            return Ok(());
        }
        println!("Ping attempt: {attempt} [ FAIL ]");
    }

    Err("Failed to get a response from Arachnid")
}

fn ping_host() -> bool {
    Command::new("ping")
        .args(["-c 1", HOST])
        .output()
        .unwrap()
        .status
        .success()
}
