pub struct Style {
    bold: bool,
    fg_color: Option<Color>,
    bg_color: Option<Color>,
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

#[allow(dead_code)]
impl Style {
    pub fn new() -> Self {
        Style {
            bold: false,
            fg_color: None,
            bg_color: None,
        }
    }

    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub fn fg(mut self, color: Color) -> Self {
        self.fg_color = Some(color);
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    pub fn apply(&self, text: &str) -> String {
        let mut codes = Vec::new();

        if self.bold {
            codes.push("1");
        }

        if let Some(color) = self.fg_color {
            codes.push(match color {
                Color::Black => "30",
                Color::Red => "31",
                Color::Green => "32",
                Color::Yellow => "33",
                Color::Blue => "34",
                Color::Magenta => "35",
                Color::Cyan => "36",
                Color::White => "37",
                Color::BrightBlack => "90",
                Color::BrightRed => "91",
                Color::BrightGreen => "92",
                Color::BrightYellow => "93",
                Color::BrightBlue => "94",
                Color::BrightMagenta => "95",
                Color::BrightCyan => "96",
                Color::BrightWhite => "97",
            });
        }

        if let Some(color) = self.bg_color {
            codes.push(match color {
                Color::Black => "40",
                Color::Red => "41",
                Color::Green => "42",
                Color::Yellow => "43",
                Color::Blue => "44",
                Color::Magenta => "45",
                Color::Cyan => "46",
                Color::White => "47",
                Color::BrightBlack => "100",
                Color::BrightRed => "101",
                Color::BrightGreen => "102",
                Color::BrightYellow => "103",
                Color::BrightBlue => "104",
                Color::BrightMagenta => "105",
                Color::BrightCyan => "106",
                Color::BrightWhite => "107",
            });
        }

        if codes.is_empty() {
            text.to_string()
        } else {
            format!("\x1b[{}m{}\x1b[0m", codes.join(";"), text)
        }
    }
}

// Convenience functions for common styles
pub fn error_style() -> Style {
    Style::new().bold().fg(Color::Red)
}

pub fn error_location_style() -> Style {
    Style::new().bold().fg(Color::Cyan)
}

pub fn error_source_style() -> Style {
    Style::new().fg(Color::White)
}

pub fn error_pointer_style() -> Style {
    Style::new().bold().fg(Color::Red)
}
