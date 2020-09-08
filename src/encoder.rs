pub enum Encoder {
    Bin,
    Oct,
    Dec,
    Hex,
}

pub use self::Encoder::{Bin, Dec, Hex, Oct};

impl Encoder {
    pub fn apply(&self, n: u8) -> String {
        match self {
            Bin => bin(n),
            Dec => dec(n),
            Hex => hex(n),
            Oct => oct(n),
        }
    }

    pub fn bytes_per_line(&self) -> usize {
        36 / (self.output_len() + 1)
    }

    pub fn output_len(&self) -> usize {
        match self {
            Bin => 8,
            Oct => 3,
            Dec => 3,
            Hex => 2,
        }
    }
}

fn bin(n: u8) -> String {
    format!("{:08b}", n)
}

fn oct(n: u8) -> String {
    format!("{:03o}", n)
}

fn dec(n: u8) -> String {
    format!("{:03}", n)
}

fn hex(n: u8) -> String {
    format!("{:02X}", n)
}
