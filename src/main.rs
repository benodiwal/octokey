use clap::{App, Arg, SubCommand};
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("OctoKey")
        .version("1.0")
        .author("Sachin Beniwal")
        .about("A tentacular tool to manage GitHub SSH keys")
        .subcommand(SubCommand::with_name("add")
            .about("Add a new SSH key for GitHub")
            .arg(Arg::with_name("key_name")
                .help("Name of the new SSH key")
                .required(true)
                .index(1))
            .arg(Arg::with_name("email")
                .short("e")
                .long("email")
                .value_name("EMAIL")
                .help("Email associated with the GitHub account")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("switch")
            .about("Switch to a different SSH key for GitHub")
            .arg(Arg::with_name("key_name")
                .help("Name of the SSH key to switch to")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("check")
            .about("Check current GitHub user"))
        .subcommand(SubCommand::with_name("list")
            .about("List all available SSH keys"))
        .get_matches();

    match matches.subcommand() {
        ("add", Some(add_matches)) => {
            let key_name = add_matches.value_of("key_name").unwrap();
            let email = add_matches.value_of("email").unwrap_or("your_email@example.com");
            add_key(key_name, email)
        },
        ("switch", Some(switch_matches)) => {
            let key_name = switch_matches.value_of("key_name").unwrap();
            switch_key(key_name)
        },
        ("check", Some(_)) => check_github_user(),
        ("list", Some(_)) => list_keys(),
        _ => Err("Invalid command. Use --help for usage information.".into()),
    }
}

fn add_key(key_name: &str, email: &str) -> Result<(), Box<dyn std::error::Error>> {
    let ssh_dir = Path::new(&std::env::var("HOME")?).join(".ssh");
    let key_path = ssh_dir.join(format!("{}", key_name));

    if key_path.exists() {
        return Err(format!("Key '{}' already exists", key_name).into());
    }

    println!("🐙 OctoKey is generating a new SSH key...");

    // Generate new SSH key
    Command::new("ssh-keygen")
        .args(&["-t", "ed25519", "-C", email, "-f", key_path.to_str().unwrap(), "-N", ""])
        .status()?;

    // Add key to ssh-agent
    Command::new("ssh-add")
        .arg(key_path.to_str().unwrap())
        .status()?;

    // Display public key for user to add to GitHub
    let public_key = fs::read_to_string(format!("{}.pub", key_path.to_str().unwrap()))?;
    println!("🎉 New SSH key generated! Add the following public key to your GitHub account:");
    println!("{}", public_key);

    Ok(())
}

fn switch_key(key_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let ssh_dir = Path::new(&std::env::var("HOME")?).join(".ssh");
    let key_path = ssh_dir.join(key_name);

    if !key_path.exists() {
        return Err(format!("Key '{}' not found", key_name).into());
    }

    println!("🔄 OctoKey is switching SSH keys...");

    // Remove all identities from ssh-agent
    Command::new("ssh-add")
        .arg("-D")
        .status()?;

    // Add the selected key to ssh-agent
    Command::new("ssh-add")
        .arg(key_path.to_str().unwrap())
        .status()?;

    println!("✅ Switched to key: {}", key_name);
    Ok(())
}

fn check_github_user() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 OctoKey is checking your GitHub user...");

    let output = Command::new("ssh")
        .args(&["-T", "git@github.com"])
        .output()?;

    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    Ok(())
}

fn list_keys() -> Result<(), Box<dyn std::error::Error>> {
    let ssh_dir = Path::new(&std::env::var("HOME")?).join(".ssh");
    
    println!("🗝️  OctoKey found these SSH keys:");
    for entry in fs::read_dir(ssh_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("pub") {
            if let Some(key_name) = path.file_stem().and_then(|s| s.to_str()) {
                println!("  - {}", key_name);
            }
        }
    }

    Ok(())
}
