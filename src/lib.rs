extern crate ansi_term;
extern crate clap;
extern crate hsl;

pub mod colours;
pub mod encoder;
mod exec;

pub use colours::colour;
pub use exec::exec;
