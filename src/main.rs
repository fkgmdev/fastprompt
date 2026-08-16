#![allow(unused)]
use chrono::Local;
use tokio::process::Command;
mod raw_colors;
use raw_colors::{Color, color};

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
    let reset = "\x1b[0m";
    if let Some(rust) = rust {
        format!(
            "\n{}{}{user} {}{}  {rust} {}{} {cwd} {}{}   {time}{} {}❯ {reset}",
            color(Color::Red, Color::Default, false),
            color(Color::Black, Color::Red, true),
            color(Color::Red, Color::Yellow, false),
            color(Color::Black, Color::Yellow, false),
            color(Color::Yellow, Color::Green, false),
            color(Color::Black, Color::Green, false),
            color(Color::Green, Color::Blue, false),
            color(Color::Black, Color::Blue, true),
            color(Color::Blue, Color::Default, false),
            color(Color::Green, Color::Default, true),
        )
    } else {
        format!(
            "\n{}{}{user} {}{}{} {cwd} {}{}   {time}{} {}❯ {reset}",
            color(Color::Red, Color::Default, false),
            color(Color::Black, Color::Red, true),
            color(Color::Red, Color::Yellow, false),
            color(Color::Yellow, Color::Green, false),
            color(Color::Black, Color::Green, false),
            color(Color::Green, Color::Blue, false),
            color(Color::Black, Color::Blue, true),
            color(Color::Blue, Color::Default, false),
            color(Color::Green, Color::Default, true),
        )
    }
}
