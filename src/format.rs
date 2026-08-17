use std::collections::HashMap;

use hex_color::HexColor;
use tokio::fs::read_to_string;
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Default,
}
pub fn color(text: &str, fg: Color, bg: Color, bold: bool) -> String {
    let mut sequence = vec!["\x1b[0"];
    if bold {
        sequence.push(";1;");
    }
    sequence.push(match fg {
        Color::Black => "30",
        Color::Red => "31",
        Color::Green => "32",
        Color::Yellow => "33",
        Color::Blue => "34",
        Color::Magenta => "35",
        Color::Cyan => "36",
        Color::White => "37",
        Color::Default => "39",
    });
    sequence.push(";");
    sequence.push(match bg {
        Color::Black => "40",
        Color::Red => "41",
        Color::Green => "42",
        Color::Yellow => "43",
        Color::Blue => "44",
        Color::Magenta => "45",
        Color::Cyan => "46",
        Color::White => "47",
        Color::Default => "49",
    });
    sequence.push("m");
    sequence.push(text);
    sequence.push("\x1b[0m");

    sequence.join("")
}

pub async fn read_config() -> String {
    read_to_string("/home/yamana/.config/fastprompt/fastprompt.config")
        .await
        .unwrap()
}
pub async fn read_colors() -> String {
    read_to_string("/home/yamana/.config/fastprompt/colors.config")
        .await
        .unwrap()
}

pub fn process_conditionals(raw: &str, active_vars: &std::collections::HashSet<&str>) -> String {
    let mut out = Vec::new();
    let mut skip = false;
    for line in raw.lines() {
        if let Some(var) = line.strip_prefix('?') {
            if var.is_empty() {
                skip = false;
                continue;
            }
            skip = !active_vars.contains(var);
            continue;
        }
        if !skip {
            out.push(line);
        }
    }
    out.join("\n")
}
fn color_rgb(
    text: &str,
    fg_color: Option<&RgbColor>,
    bg_color: Option<&RgbColor>,
    bold: bool,
) -> String {
    let mut seq = Vec::new();
    seq.push("\x1b[0");
    if bold {
        seq.push(";1");
    }
    seq.push("m");
    let set_fg = if let Some(c) = fg_color {
        &format!("\x1b[38;2;{};{};{}m", c.r, c.g, c.b)
    } else {
        "\x1b[39m"
    };
    let set_bg = if let Some(c) = bg_color {
        &format!("\x1b[48;2;{};{};{}m", c.r, c.g, c.b)
    } else {
        "\x1b[49m"
    };

    seq.push(set_fg);
    seq.push(set_bg);
    seq.push(text);
    seq.push("\x1b[0m");
    seq.join("")
}

struct RgbColor {
    r: i32,
    g: i32,
    b: i32,
}
pub fn render(raw: &str, colors: &str) -> String {
    let mut colorset = HashMap::new();
    for line in colors.lines() {
        let args: Vec<&str> = line.split("=").collect();
        let rgb = HexColor::parse(args[1]).unwrap();
        let color = RgbColor {
            r: rgb.r as i32,
            g: rgb.g as i32,
            b: rgb.b as i32,
        };
        colorset.insert(args[0], color);
    }
    let mut format = Vec::new();
    for line in raw.lines() {
        let args: Vec<&str> = line.split(';').collect();
        let bold = args.len() == 4 && args[3] == "bold";
        // let fg_color = match args[1] {
        //     "black" => Color::Black,
        //     "red" => Color::Red,
        //     "green" => Color::Green,
        //     "yellow" => Color::Yellow,
        //     "blue" => Color::Blue,
        //     "magenta" => Color::Magenta,
        //     "cyan" => Color::Cyan,
        //     "white" => Color::White,
        //     "default" => Color::Default,
        //     _ => Color::Default,
        // };
        // let bg_color = match args[2] {
        //     "black" => Color::Black,
        //     "red" => Color::Red,
        //     "green" => Color::Green,
        //     "yellow" => Color::Yellow,
        //     "blue" => Color::Blue,
        //     "magenta" => Color::Magenta,
        //     "cyan" => Color::Cyan,
        //     "white" => Color::White,
        //     "default" => Color::Default,
        //     _ => Color::Default,
        // };
        let fg_color = colorset.get(args[1]);
        let bg_color = colorset.get(args[2]);

        format.push(color_rgb(args[0], fg_color, bg_color, bold));
    }
    format.join("")
}
