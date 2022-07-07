extern crate smallvec;
extern crate textwrap;
extern crate unicode_width;

use smallvec::*;
use std::io::{Result, Write};
use std::iter::repeat;
use std::str;
use textwrap::fill;
use unicode_width::UnicodeWidthStr;
const ENDSL: &[u8] = b"| ";
const ENDSR: &[u8] = b" |\n";
#[cfg(not(feature = "clippy"))]
const AMOGUS: &str = r#"
        \
         \
         ⠀\⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
         ⠀⠀\⠀⠀⠀⠀⠀⠀⠀⠀⣠⣴⣶⣿⣿⣷⣶⣄⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀
         ⠀⠀⠀\⠀⠀⠀⠀⠀⣰⣾⣿⣿⡿⢿⣿⣿⣿⣿⣿⣿⣿⣷⣦⡀⠀⠀⠀⠀⠀
         ⠀⠀⠀⠀\⠀⠀⢀⣾⣿⣿⡟⠁⣰⣿⣿⣿⡿⠿⠻⠿⣿⣿⣿⣿⣧⠀⠀⠀⠀
         ⠀⠀⠀⠀⠀\⠀⣾⣿⣿⠏⠀⣴⣿⣿⣿⠉⠀⠀⠀⠀⠀⠈⢻⣿⣿⣇⠀⠀⠀
         ⠀⠀⠀⠀⢀⣠⣼⣿⣿⡏⠀⢠⣿⣿⣿⠇⠀⠀⠀⠀⠀⠀⠀⠈⣿⣿⣿⡀⠀⠀
         ⠀⠀⠀⣰⣿⣿⣿⣿⣿⡇⠀⢸⣿⣿⣿⡀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⡇⠀⠀
         ⠀⠀⢰⣿⣿⡿⣿⣿⣿⡇⠀⠘⣿⣿⣿⣧⠀⠀⠀⠀⠀⠀⢀⣸⣿⣿⣿⠁⠀⠀
         ⠀⠀⣿⣿⣿⠁⣿⣿⣿⡇⠀⠀⠻⣿⣿⣿⣷⣶⣶⣶⣶⣶⣿⣿⣿⣿⠃⠀⠀⠀
         ⠀⢰⣿⣿⡇⠀⣿⣿⣿⠀⠀⠀⠀⠈⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠁⠀⠀⠀⠀
         ⠀⢸⣿⣿⡇⠀⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠉⠛⠛⠛⠉⢉⣿⣿⠀⠀⠀⠀⠀⠀
         ⠀⢸⣿⣿⣇⠀⣿⣿⣿⠀⠀⠀⠀⠀⢀⣤⣤⣤⡀⠀⠀⢸⣿⣿⣿⣷⣦⠀⠀⠀
         ⠀⠀⢻⣿⣿⣶⣿⣿⣿⠀⠀⠀⠀⠀⠈⠻⣿⣿⣿⣦⡀⠀⠉⠉⠻⣿⣿⡇⠀⠀
         ⠀⠀⠀⠛⠿⣿⣿⣿⣿⣷⣤⡀⠀⠀⠀⠀⠈⠹⣿⣿⣇⣀⠀⣠⣾⣿⣿⡇⠀⠀
         ⠀⠀⠀⠀⠀⠀⠀⠹⣿⣿⣿⣿⣦⣤⣤⣤⣤⣾⣿⣿⣿⣿⣿⣿⣿⣿⡟⠀⠀⠀
         ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠻⢿⣿⣿⣿⣿⣿⣿⠿⠋⠉⠛⠋⠉⠉⠁⠀⠀⠀⠀
         ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠉⠉⠁⠀⠀⠀⠀⠀⠀
"#;
const NEWLINE: u8 = b'\n';
const DASH: u8 = b'-';
const UNDERSCORE: u8 = b'_';

// A decent number for SmallVec's Buffer Size, not too large
// but also big enough for most inputs
const BUFSIZE: usize = 2048;

pub fn say<W>(input: &[u8], max_width: usize, writer: &mut W) -> Result<()>
where
    W: Write,
{
    // Final output is stored here
    let mut write_buffer = SmallVec::<[u8; BUFSIZE]>::new();

    // Let textwrap work its magic
    let wrapped = fill(
        str::from_utf8(input).map_err(|_| std::io::ErrorKind::InvalidData)?,
        max_width,
    );

    let lines: Vec<&str> = wrapped.lines().collect();

    let line_count = lines.len();
    let actual_width = longest_line(&lines);

    let mut top_bar_buffer: Vec<u8> = repeat(UNDERSCORE).take(actual_width + 2).collect();
    top_bar_buffer.insert(0, b' ');

    let mut bottom_bar_buffer: Vec<u8> = repeat(DASH).take(actual_width + 2).collect();
    bottom_bar_buffer.insert(0, b' ');

    write_buffer.extend_from_slice(&top_bar_buffer);
    write_buffer.push(NEWLINE);

    for (current_line, line) in lines.into_iter().enumerate() {
        if line_count == 1 {
            write_buffer.extend_from_slice(b"< ");
        } else if current_line == 0 {
            write_buffer.extend_from_slice(b"/ ");
        } else if current_line == line_count - 1 {
            write_buffer.extend_from_slice(b"\\ ");
        } else {
            write_buffer.extend_from_slice(ENDSL);
        }

        let line_len = UnicodeWidthStr::width(line);
        write_buffer.extend_from_slice(line.as_bytes());
        for _i in line_len..actual_width {
            write_buffer.extend_from_slice(b" ");
        }

        if line_count == 1 {
            write_buffer.extend_from_slice(b" >\n");
        } else if current_line == 0 {
            write_buffer.extend_from_slice(b" \\\n");
        } else if current_line == line_count - 1 {
            write_buffer.extend_from_slice(b" /\n");
        } else {
            write_buffer.extend_from_slice(ENDSR);
        }
    }

    write_buffer.extend_from_slice(&bottom_bar_buffer);
    // #[cfg(feature = "clippy")]
    // write_buffer.extend_from_slice(CLIPPY);
    #[cfg(not(feature = "clippy"))]
    write_buffer.extend_from_slice(AMOGUS.as_bytes());
    writer.write_all(&write_buffer)?;

    Ok(())
}

fn longest_line(lines: &[&str]) -> usize {
    let mut max_width = 0;
    for line in lines {
        let line_width = UnicodeWidthStr::width(line.to_owned());
        if line_width > max_width {
            max_width = line_width;
        }
    }
    max_width
}

//  EXMAPLE
//  use std::io::{stdout, BufWriter};
//  fn main() {
//     let out = b"Sussy Bakas!";
//      let width = 8;
//
//      let mut writer = BufWriter::new(stdout());
//      say(out, width, &mut writer).unwrap();
//  }
