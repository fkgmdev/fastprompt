#![allow(unused)]
use chrono::Local;
use std::collections::HashSet;
use tokio::{fs::read_to_string, process::Command};
mod format;

async fn get_user() -> String {
    match Command::new("whoami").output().await {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
        Err(_) => String::from("user"),
    }
}
fn get_cwd(user: &str) -> String {
    let cwd = match std::env::current_dir() {
        Ok(path) => path.to_string_lossy().to_string(),
        Err(_) => return String::from("path"),
    };
    let usr_path = format!("/home/{}", user.trim());
    if cwd.starts_with(&usr_path) {
        cwd.replace(&usr_path, "~")
    } else {
        cwd
    }
}
async fn get_rust() -> Option<String> {
    let mut current = std::env::current_dir().ok()?;
    loop {
        let cargo = current.join("Cargo.toml");
        if cargo.exists() {
            match Command::new("rustc").arg("--version").output().await {
                Ok(output) => {
                    return Some(
                        " ".to_string()
                            + String::from_utf8_lossy(&output.stdout)
                                .to_string()
                                .split_whitespace()
                                .nth(1)
                                .unwrap_or("rust"),
                    );
                }
                Err(_) => return None,
            };
        }
        if !current.pop() {
            break;
        }
    }
    None
}
async fn get_os() -> String {
    let os_release = read_to_string("/etc/os-release")
        .await
        .unwrap_or("ID=linux".to_string());
    match os_release
        .lines()
        .find(|line| line.starts_with("ID="))
        .and_then(|line| line.split('=').nth(1))
        .unwrap_or("linux")
    {
        "fedora" => "󰣛".to_string(),
        "linux" => "󰌽".to_string(),
        "arch" => "󰣇".to_string(),
        _ => "".to_string(),
    }
}
#[tokio::main]
async fn main() {
    let (raw_config, user, rust, os) =
        tokio::join!(format::read_config(), get_user(), get_rust(), get_os());
    let cwd = get_cwd(&user);
    let time = Local::now().format("%H:%M").to_string();

    let mut active = HashSet::from(["user", "cwd", "time", "os"]);
    if rust.is_some() {
        active.insert("rust");
    }

    let processed = format::process_conditionals(&raw_config, &active);
    let colored = format::render(&processed);
    println!("{}", make_prompt(&user, &cwd, rust, &time, &colored, &os));
}

fn make_prompt(
    user: &str,
    cwd: &str,
    rust: Option<String>,
    time: &str,
    format: &str,
    os: &str,
) -> String {
    let rust_str = rust.unwrap_or_default();
    format
        .replace("$space", " ")
        .replace("$user", user)
        .replace("$rust", &rust_str)
        .replace("$time", time)
        .replace("$cwd", cwd)
        .replace("$newline", "\n")
        .replace("$os", os)
}
