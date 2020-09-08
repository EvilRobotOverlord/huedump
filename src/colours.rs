use ansi_term::Colour::RGB;
use hsl::HSL;

pub fn colour(n: u8, text: String) -> String {
    let hsl = HSL {
        h: (n as f64) * 200. / 255.,
        s: 240.,
        l: 140.,
    }.to_rgb();

    RGB(hsl.0, hsl.1, hsl.2).paint(text).to_string()
}

pub fn leave_alone(_n: u8, text: String) -> String {
    text
}
