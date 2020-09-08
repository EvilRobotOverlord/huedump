use clap::App;
use encoder::{Bin, Dec, Hex, Oct, Encoder};

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

pub fn print_bytes<T>(colour: T, filename: &Path, encoding: Encoder) -> io::Result<()>
where
    T: Fn(u8, String) -> String,
{
    let mut file = File::open(filename)?;
    let bytes = encoding.bytes_per_line();
    let output_len = encoding.output_len();
    let mut buf = vec![0; bytes];
    let mut byte = 0;

    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }

        print!("{}", colour(255, format!("{:10}: ", byte)));

        for i in 0..n {
            print!("{} ", colour(buf[i], encoding.apply(buf[i] as u8)));
        }
        for _ in 0..bytes - n {
            print!("{} ", colour(0, "_".repeat(output_len)));
        }

        println!();
        byte += bytes;
    }

    Ok(())
}

pub fn exec<T>(colour: T) -> Result<(), Box<Error>>
where
    T: Fn(u8, String) -> String,
{
    let matches = App::new("huedump")
                    .version("0.1.0")
                    .about("Creates the dump of a file, colouring the bytes based on their value so that it looks better")
                    .args_from_usage(
                        "<FILE>                     'File to dump'
                        -e, --encoding=[ENCODING]   'Encoding (`bin`, `oct`, `hex` or `dec`) to print bytes in'"
                    ).get_matches();

    if let Some(filename) = matches.value_of("FILE") {
        let encoding = matches.value_of("encoding").unwrap_or("hex");
        let encoder = match encoding {
            "bin" => Bin,
            "dec" => Dec,
            "hex" => Hex,
            "oct" => Oct,
            _ => Hex,
        };

        let filename = Path::new(filename);
        println!("{}", colour(127, filename.canonicalize()?.to_string_lossy().into_owned()));

        print_bytes(colour, filename, encoder)?;
        Ok(())
    } else {
        Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Missing filename argument. Use `huedump --help` for help.",
        )))
    }
}
