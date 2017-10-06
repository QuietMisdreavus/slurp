//! A collection of convenience methods around loading files into various containers.
//!
//! This crate contains a few small wrapper methods around common operations in the standard
//! library for loading files. The `Read` trait is a little cumbersome if you don't want to bother
//! keeping up with a buffer, or if you're only loading one file. Here, you can wrap up all that
//! boilerplate into little one-off functions!
//!
//! To read a file into a string:
//!
//! ```no_run
//! let my_file = slurp::read_all_to_string("myfile.txt").unwrap();
//! ```
//!
//! To read a file into a byte vector:
//!
//! ```no_run
//! let my_file = slurp::read_all_bytes("myfile.txt").unwrap();
//! ```
//!
//! Or, to read a file into a Vec where each element is a different line:
//!
//! ```no_run
//! let my_file: Vec<String> = slurp::read_all_lines("myfile.txt").unwrap();
//! ```
//!
//! There's also an iterator to lazily load the lines, though it's mainly a wrapper over
//! `io::BufReader`:
//!
//! ```no_run
//! for line in slurp::iterate_all_lines("myfile.txt") {
//!     let line = line.unwrap();
//! }
//! ```

#![deny(warnings, missing_docs)]

use std::io::{self, Read, BufRead};
use std::fs::File;
use std::path::{Path, PathBuf};

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

/// Returns an iterator over the lines in the file at the given filename.
///
/// Note that this iterator lazily opens the file - it won't touch the filesystem until you start
/// iterating.
pub fn iterate_all_lines<P: AsRef<Path>>(filename: P) -> Lines {
    Lines {
        filename: filename.as_ref().to_path_buf(),
        iter: None,
    }
}

/// Reads the lines of the file at the given filename into a new collection of Strings.
pub fn read_all_lines<P: AsRef<Path>>(filename: P) -> io::Result<Vec<String>> {
    iterate_all_lines(filename).collect()
}

/// Iterator over the lines of a file.
///
/// See [`iterate_all_lines`] for details.
///
/// [`iterate_all_lines`]: fn.iterate_all_lines.html
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Lines {
    filename: PathBuf,
    iter: Option<io::Lines<io::BufReader<File>>>,
}

impl Iterator for Lines {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter.is_none() {
            match File::open(&self.filename) {
                Ok(f) => self.iter = Some(io::BufReader::new(f).lines()),
                Err(e) => return Some(Err(e)),
            }
        }

        self.iter.as_mut().and_then(|i| i.next())
    }
}
