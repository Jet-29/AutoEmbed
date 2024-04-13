use std::process::Command;

const HOST: &str = "Arachnid";
const WAIT_TIME: u64 = 2;

const EMBEDDED_DIR: &str = "dev/rust/Microbit";

fn main() -> Result<(), &'static str> {
    // Get project to embed.
    let mut args = std::env::args();
    args.next(); // Skip the first arg.
    let project_name = args.next().ok_or("Specify the project")?;

    let home_dir = std::env::var("HOME").or(Err("No HOME env var"))?;
    let home_path = std::path::Path::new(&home_dir);
    let embedded_path = home_path.join(EMBEDDED_DIR);
    let project_path = embedded_path.join("projects").join(&project_name);

    // Check the project folder is valid
    if !project_path.exists() {
        return Err("Project specified can not be found");
    }

    // Check HOST is online
    if !ping_host() {
        println!("Host not found attempting to start");
        attempt_to_turn_on_host(10)?;
    }

    // Begin commands
    // Sync the embedded workspace
    Command::new("rsync")
        .args(["-a", "--delete"])
        .arg(embedded_path.into_os_string())
        .arg(format!("{HOST}:embedded/"))
        .spawn()
        .or(Err("Failed to run rsync command"))?
        .wait()
        .or(Err("Failed to wait for rsync"))?;

    // probe_flash
    Command::new("ssh")
        .args(["-t", HOST])
        .arg(format!(
            ". $HOME/.cargo/env; cd embedded/Microbit/projects/{project_name}; ~/embedded/probe_embed"
        ))
        .spawn()
        .or(Err("Failed to run embed command"))?
        .wait()
        .or(Err("Failed to wait for ssh connection"))?;

    Ok(())
}

// Attempts to run the vmware start command for my machine.
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

// Pings the host ones and returns if it was successful
fn ping_host() -> bool {
    Command::new("ping")
        .args(["-c 1", HOST])
        .output()
        .unwrap()
        .status
        .success()
}
