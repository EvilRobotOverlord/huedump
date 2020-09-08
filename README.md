# huedump
A file dump creator that cycles through hues to colour each byte! Because why not?

![huedump in action](https://user-images.githubusercontent.com/23280582/41038562-3f9e6eec-69b4-11e8-9a6e-2a73a8e5f223.gif)

Special mention to [@STPR](https://github.com/STPR) for deciphering the text in that GIF and [posting it here](https://github.com/EvilRobotOverlord/huedump/issues/2) (I must say, I'm impressed; I didn't expect anyone to automate the process like that).

## Installation
1. Installl [Rust](https://www.rust-lang.org)
2. Run `cargo install huedump` (It's that simple!)

## Usage
Thanks to this great library called [`clap`](https://crates.io/crates/clap), you can just do `huedump --help` for detailed help. Here's a short version:
```sh
huedump <FILE> [--encoding=ENCODING]
```

Where `ENCODING` is anyone of:
- `hex` for hexadecimal (default)
- `bin` for binary
- `dec` for decimal
- `oct` for octal

`<FILE>` is the path to the file you want to dump. Any path works fine!

Here's how you dump `~/file.dat` in decimal (LINUX).
```sh
huedump ~/file.dat --encoding=dec
```

Here's the same thing in Windows.
```sh
huedump C:/Users/<your username here>/file.dat --encoding=dec
```

This works on files in the same directory you're in too. Relative paths are allowed.
```sh
cd ~
huedump file.dat --encoding=dec
```
