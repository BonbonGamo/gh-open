use std::fs::{self, File};
use std::io::Read;
use std::io::{self, Write};
use std::process::Command;

static SEPARATOR: &str = " ";
static ACCOUNT_ROW_MARK: &str = "__account";

fn exit_with_message(message: &str) {
    println!("{}", message);
    std::process::exit(0x0010);
}

fn read_config_file() -> Result<String, io::Error> {
    let mut f = File::open("config.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn get_url(account: String, repo: String) -> String {
    let mut url: String = String::from("");
    if account == ACCOUNT_ROW_MARK {
        exit_with_message("Illegal account name '__account'");
    }
    match read_config_file() {
        Ok(config_file) => {
            let rows: Vec<&str> = config_file.split("\n").collect();
            for row in rows.iter() {
                let row_data: Vec<&str> = row.split(SEPARATOR).collect();
                if row_data[0] == ACCOUNT_ROW_MARK {
                    url.push_str(row_data[2]);
                }
            }

            for row in rows.iter() {
                let row_data: Vec<&str> = row.split(SEPARATOR).collect();
                if row_data[0] == account && row_data[1] == repo {
                    url.push_str(row_data[2]);
                    return url;
                }
            }
            url
        }
        Err(_) => "".to_string(),
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
    let row = format!("\n{} {}", repo_name, repo_target);
    add_to_config(row).expect("Could not write");
}

fn main() {
    let root_or_command = std::env::args().nth(1).expect("no command or root given");

    if root_or_command == "add-repo".to_string() {
        on_add_repo();
        exit_with_message("Repository added!");
    }

    let repo = std::env::args().nth(2).expect("no repo given");

    let url = get_url(root_or_command, repo);
    println!("url {:?}", url);

    let output = Command::new("open")
        .args(&[&url])
        .output()
        .expect("failed to execute process");

    println!("root {:?}", output.stdout);
}
