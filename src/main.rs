use std::fs::{self, File};
use std::io::Read;
use std::io::{self, Write};
use std::process::Command;

static SEPARATOR: &str = " ";
static ACCOUNT_ROW_MARK: &str = "__account";
static CONFIG_PATH: &str = "config.txt";

fn exit_with_message(message: &str) {
    println!("{}", message);
    std::process::exit(0x0010);
}

fn get_arg(index: usize, error_message: &str) -> String {
    std::env::args().nth(index).expect(error_message)
}

fn read_config_file() -> Result<String, io::Error> {
    let mut f = File::open(CONFIG_PATH)?;
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
        .open(CONFIG_PATH)
        .unwrap();

    file.write_all(row.as_bytes())?;
    Ok(())
}

fn on_add_repo() {
    let repo_name: String = std::env::args().nth(2).expect("no repo name given");
    let repo_target: String = std::env::args().nth(3).expect("no repo target given");
    let row = format!("\n{}{}{}", repo_name, SEPARATOR, repo_target);
    add_to_config(row).expect("Could not write");
}

fn validate_account_name_and_url() {
    let config = read_config_file().expect("Config file missing");
    let rows: Vec<&str> = config.split("\n").collect();
    let name = get_arg(2, "No name for account provided");
    let url = get_arg(3, "No url for account provided");
    for row in rows.iter() {
        let row_data: Vec<&str> = row.split(SEPARATOR).collect();
        if row_data[0] == ACCOUNT_ROW_MARK && (row_data[1] == name) {
            let msg = format!("Account name {} is already taken", name);
            exit_with_message(&msg)
        }
        if row_data[0] == ACCOUNT_ROW_MARK && (row_data[2] == url) {
            let msg = format!("Account with url {} exists with name {}", url, row_data[1]);
            exit_with_message(&msg)
        }
    }
}

fn on_add_account() {
    let account_name: String = get_arg(2, "No account name provided");
    let account_target: String = get_arg(3, "No github account URL provided");
    let row = format!("\n{}{}{}", account_name, SEPARATOR, account_target);
    add_to_config(row).expect("Could not write");
}

fn main() {
    let root_or_command = std::env::args().nth(1).expect("no command or root given");

    if root_or_command == "add-repo".to_string() {
        on_add_repo();
        exit_with_message("Repository added!");
    }

    if root_or_command == "add-account".to_string() {
        validate_account_name_and_url();
        on_add_account();
        exit_with_message("Account added")
    }

    let repo = std::env::args().nth(2).expect("no repo given");

    let url = get_url(root_or_command, repo);

    let output = Command::new("open")
        .args(&[&url])
        .output()
        .expect("failed to execute process");

    println!("Opening Github: {}", output.stdout[0]);
}
