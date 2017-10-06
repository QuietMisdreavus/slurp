//! A collection of convenience methods around reading and writing files.
//!
//! This crate contains a few small wrapper methods around common operations in the standard
//! library for interacting with files. The `Read` trait is a little cumbersome if you don't want
//! to bother keeping up with a buffer, or if you're only loading one file. Here, you can wrap up
//! all that boilerplate into little one-off functions!
//!
//! ## Reading files
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
//!
//! ## Writing files
//!
//! There are also wrappers for writing to files! They're split up into `write_all_*` and
//! `append_all_*` versions based on whether you want to overwrite or replace existing files.
//!
//! To write a string to a file:
//!
//! ```no_run
//! let content = "sup".to_string();
//!
//! slurp::write_all_text("myfile.txt", &content).unwrap();
//! ```
//!
//! To write a byte buffer to a file:
//!
//! ```no_run
//! let content = vec![1,2,3];
//!
//! slurp::write_all_bytes("myfile.txt", &content).unwrap();
//! ```
//!
//! You can even write a collection of strings to a file, one per line:
//!
//! ```no_run
//! let content = vec!["hey", "yo", "sup"];
//!
//! slurp::write_all_lines("myfile.txt", &content).unwrap();
//! ```

#![deny(warnings, missing_docs)]

use std::io::{self, Read, BufRead, Write};
use std::fs::{File, OpenOptions};
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

/// Writes the given text to the file at the given filename, overwriting the file if it already
/// exists.
pub fn write_all_text<P: AsRef<Path>>(filename: P, text: &str) -> io::Result<()> {
    write_all_bytes(filename, text.as_bytes())
}

/// Writes the given bytes to the file at the given filename, overwriting the file if it already
/// exists.
pub fn write_all_bytes<P: AsRef<Path>>(filename: P, bytes: &[u8]) -> io::Result<()> {
    let mut file = File::create(filename)?;

    file.write_all(bytes)?;

    file.flush()
}

/// Writes the given set of lines to the file at the given filename, overwriting the file if it
/// already exists.
///
/// ## Errors
///
/// If this function encounters an error midway through the iterator, the file will be left
/// partially filled.
pub fn write_all_lines<P: AsRef<Path>, I: IntoIterator<Item=S>, S: AsRef<str>>
(
    filename: P,
    lines: I
)
    -> io::Result<()>
{
    let mut file = File::create(filename)?;

    for line in lines {
        writeln!(&mut file, "{}", line.as_ref())?;
    }

    file.flush()
}

/// Writes the given text to the file at the given filename, creating it if it doesn't exist or
/// appending to the end if it does.
pub fn append_all_text<P: AsRef<Path>>(filename: P, text: &str) -> io::Result<()> {
    append_all_bytes(filename, text.as_bytes())
}

/// Writes the given bytes to the file at the given filename, creating it if it doesn't exist or
/// appending to the end if it does.
pub fn append_all_bytes<P: AsRef<Path>>(filename: P, bytes: &[u8]) -> io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(filename)?;

    file.write_all(bytes)?;

    file.flush()
}

/// Writes the given set of lines to the file at the given filename, creating it if it doesn't
/// exist or appending to the end if it does.
///
/// ## Errors
///
/// If this function encounters an error midway through the iterator, the file will be left
/// partially filled.
pub fn append_all_lines<P: AsRef<Path>, I: IntoIterator<Item=S>, S: AsRef<str>>
(
    filename: P,
    lines: I
)
    -> io::Result<()>
{
    let mut file = OpenOptions::new().create(true).append(true).open(filename)?;

    for line in lines {
        writeln!(&mut file, "{}", line.as_ref())?;
    }

    file.flush()
}
