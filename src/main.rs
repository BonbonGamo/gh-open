use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::io::{self, Write};
use std::process::Command;

fn read_config_file() -> Result<String, io::Error> {
    let mut f = File::open("config.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn get_config() -> HashMap<String, String> {
    let mut config: HashMap<String, String> = HashMap::new();
    match read_config_file() {
        Ok(config_file) => {
            let config_parts: Vec<&str> = config_file.split("#repos").collect();
            let root: Vec<&str> = config_parts[0].split("\n").collect();
            let mut repos: Vec<&str> = config_parts[1].split("\n").collect();

            repos.remove(0);

            for repo in repos.iter() {
                let key_and_value: Vec<&str> = repo.split(":").collect();
                config.insert(key_and_value[0].to_string(), key_and_value[1].to_string());
            }
            config.insert("root".to_string(), root[1].to_string());
            config
        }
        Err(_) => HashMap::new(),
    }
}

fn add_to_config(row: String) -> Result<(), io::Error> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true) // This is needed to append to file
        .open("config.txt")
        .unwrap();

    file.write_all(row.as_bytes())?;
    Ok(())
}

fn on_add_repo() {
    let repo_name: String = std::env::args().nth(2).expect("no repo name given");
    let repo_target: String = std::env::args().nth(3).expect("no repo target given");
    let row = format!("\n{}:{}", repo_name, repo_target);
    add_to_config(row).expect("Could not write");
}

fn main() {
    let repo_or_command = std::env::args().nth(1).expect("no command or repo given");

    if repo_or_command == "add-repo".to_string() {
        on_add_repo();
        std::process::exit(0x0010);
    }

    let config = get_config();
    let root = config
        .get("root")
        .expect("Config file corrupted. Run config reset");

    let repo = config.get(&repo_or_command).expect("Unknown repo");

    let url = format!("{}{}", root, repo);
    println!("url {:?}", url);
    let output = Command::new("open")
        .args(&[&url])
        .output()
        .expect("failed to execute process");

    println!("root {:?}", output.stdout);
}
