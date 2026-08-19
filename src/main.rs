#![allow(unused)]
use chrono::Local;
use std::{collections::HashSet, os::linux::raw::stat};
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
#[derive(Debug, PartialEq)]
enum Git {
    Modified(String),
    Staged(String),
    //Committed,
    Clean(String),
}
async fn get_git() -> Option<Git> {
    let mut current = std::env::current_dir().unwrap();
    loop {
        if current.join(".git").exists() {
            let branch = read_to_string(current.join(".git/HEAD"))
                .await
                .unwrap_or("unknown/unknown".to_string())
                .split("/")
                .last()
                .unwrap_or("unknown")
                .trim()
                .to_string();
            if let Ok(status_str) = Command::new("git")
                .args(&["status", "--porcelain"])
                .output()
                .await
            {
                if String::from_utf8_lossy(&status_str.stdout)
                    .trim()
                    .is_empty()
                {
                    return Some(Git::Clean(branch));
                }
                for line in String::from_utf8_lossy(&status_str.stdout).lines() {
                    if line.trim().starts_with("M")
                        || line.trim().starts_with("A")
                        || line.trim().starts_with("D")
                    {
                        return Some(Git::Staged(branch));
                    } else if line.trim().starts_with(" ") {
                        return Some(Git::Modified(branch));
                    }
                }
                return None;
            } else {
                return None;
            }
        }
        if !current.pop() {
            break;
        }
    }
    None
}
#[tokio::main]
async fn main() {
    let (raw_config, colors, user, rust, os, git) = tokio::join!(
        format::read_config(),
        format::read_colors(),
        get_user(),
        get_rust(),
        get_os(),
        get_git()
    );
    let cwd = get_cwd(&user);
    let time = Local::now().format("%H:%M").to_string();

    let mut active = HashSet::from(["user", "cwd", "time", "os"]);
    if rust.is_some() {
        active.insert("rust");
    }
    if git.is_some() {
        active.insert("git");
    }

    let processed = format::process_conditionals(&raw_config, &active);
    let colored = format::render(&processed, &colors);
    println!(
        "{}",
        make_prompt(&user, &cwd, rust, &time, &colored, &os, git)
    );
}

fn make_prompt(
    user: &str,
    cwd: &str,
    rust: Option<String>,
    time: &str,
    format: &str,
    os: &str,
    git: Option<Git>,
) -> String {
    let rust_str = rust.unwrap_or_default();
    let (git_str, branch) = if let Some(git_status) = git {
        match git_status {
            Git::Modified(branch) => ("!", branch),
            Git::Staged(branch) => ("+", branch),
            Git::Clean(branch) => ("✓", branch),
        }
    } else {
        ("?", "unknown".to_string())
    };
    format
        .replace("$space", " ")
        .replace("$user", user)
        .replace("$rust", &rust_str)
        .replace("$time", time)
        .replace("$cwd", cwd)
        .replace("$newline", "\n")
        .replace("$os", os)
        .replace("$gitstatus", git_str)
        .replace("$gitbranch", &branch)
}
