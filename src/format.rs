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

pub async fn config() -> String {
    let config = read_to_string("/home/yamana/.config/fastprompt/fastprompt.config")
        .await
        .unwrap();

    let mut format = Vec::new();
    for line in config.lines() {
        let args: Vec<&str> = line.split(';').collect();
        let bold = if args.len() == 4 && args[3] == "bold" {
            true
        } else {
            false
        };
        if args.len() == 1 {
            format.push(args[0].to_string());
            continue;
        }
        let fg_color = match args[1] {
            "black" => Color::Black,
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "magenta" => Color::Magenta,
            "cyan" => Color::Cyan,
            "white" => Color::White,
            "default" => Color::Default,
            _ => Color::Default,
        };
        let bg_color = match args[2] {
            "black" => Color::Black,
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "magenta" => Color::Magenta,
            "cyan" => Color::Cyan,
            "white" => Color::White,
            "default" => Color::Default,
            _ => Color::Default,
        };

        format.push(color(args[0], fg_color, bg_color, bold));
    }

    format.join("")
}
