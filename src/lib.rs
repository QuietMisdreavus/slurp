//! A collection of convenience methods around loading files into various containers.

#![deny(warnings, missing_docs)]

use std::io::{self, Read, BufRead};
use std::fs::File;
use std::path::Path;

/// Reads the file at the given filename into a new String.
pub fn read_all_to_string<P: AsRef<Path>>(filename: P) -> io::Result<String> {
    let mut out = String::new();
    let mut file = File::open(filename)?;

    file.read_to_string(&mut out)?;

    Ok(out)
}

/// Reads the file at the given filename into a new byte vector.
pub fn read_all_bytes<P: AsRef<Path>>(filename: P) -> io::Result<Vec<u8>> {
    let mut out = Vec::new();
    let mut file = File::open(filename)?;

    file.read_to_end(&mut out)?;

    Ok(out)
}

/// Reads the lines of the file at the given filename into a new collection of Strings.
pub fn read_all_lines<P: AsRef<Path>>(filename: P) -> io::Result<Vec<String>> {
    io::BufReader::new(File::open(filename)?).lines().collect()
}
