#![allow(unused)]
use chrono::Local;
use tokio::process::Command;

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
                        String::from_utf8_lossy(&output.stdout)
                            .to_string()
                            .split_whitespace()
                            .nth(1)
                            .unwrap_or("rust")
                            .to_string(),
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
#[tokio::main]
async fn main() {
    let (user, rust) = tokio::join!(get_user(), get_rust());
    let cwd = get_cwd(&user);
    let time = Local::now().format("%H:%M").to_string();
    println!("{}", make_prompt(&user, &cwd, rust, time));
}

fn make_prompt(user: &str, cwd: &str, rust: Option<String>, time: String) -> String {
    if let Some(rust) = rust {
        format!(
            "\x1b[0;31m\x1b[0;30;41m{user} \x1b[0;31;43m\x1b[0;30;43m  {rust} \x1b[0;33;42m\x1b[0;30;42m {cwd} \x1b[0;32;44m \x1b[0;30;44m{time}\x1b[0;34m\x1b[0m >"
        )
    } else {
        format!(
            "\x1b[0;31m\x1b[0;30;41m{user} \x1b[0;31;42m\x1b[0;30;42m {cwd} \x1b[0;32;44m \x1b[0;30;44m{time}\x1b[0;34m\x1b[0m >"
        )
    }
}
