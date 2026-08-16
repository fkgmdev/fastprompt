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

pub fn color(fg: Color, bg: Color, bold: bool) -> String {
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

    sequence.join("")
}
