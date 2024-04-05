use std::process::Command;

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

    // Begin commands
    // rsync
    Command::new("rsync")
        .args(["-a", "--delete"])
        .arg(format!("./../{current_dir}"))
        .arg("Arachnid:embedded/")
        .spawn()
        .or(Err("Failed to run rsync command"))?;
    // TODO This could fail due to the Arachnid machine being offline
    // I can make it check that first and power it on if not

    Ok(())
}

fn from_target_to_chip(target: &str) -> Result<&'static str, &'static str> {
    match target {
        "thumbv6m-none-eabi" => Ok("nRF51822_xxAA"),
        _ => Err("Target not recognised as a specific chip"),
    }
}
