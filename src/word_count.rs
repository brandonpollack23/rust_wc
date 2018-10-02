use config::Config;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::ErrorKind;
use std::path::Path;
use std::io::BufRead;
use std::io::Read;
use std::fmt::Write;

#[derive(Debug)]
pub struct Counts {
    bytes: usize,
    chars: usize,
    lines: usize,
    words: usize,
    max_length: usize,
}

impl Counts {
    fn zeros() -> Counts {
        Counts {
            bytes: 0,
            chars: 0,
            lines: 0,
            words: 0,
            max_length: 0,
        }
    }

    pub fn create_result_string(&self, config: &Config) -> String {
        let mut output = String::from(" ");

        if config.print_lines {
            write!(&mut output, "{} ", self.lines);
        }
        if config.print_words {
            write!(&mut output, "{} ", self.words);
        }
        if config.print_bytes {
            write!(&mut output, "{} ", self.bytes);
        }
        if config.print_chars {
            write!(&mut output, "{} ", self.chars);
        }
        if config.print_max_length {
            write!(&mut output, "{} ", self.max_length);
        }

        // Remove extra space.
        output.pop();

        output
    }
}

pub fn run_coreutil_wc(path: &str) -> io::Result<Counts> {
    let file_reader = open(path)?;

    let mut counts = Counts::zeros();

    for current_line in file_reader.lines() {
        line_counts(&current_line.unwrap(), &mut counts);
    }

    Ok(counts)
}

fn open(path: &str) -> io::Result<BufReader<Box<Read>>> {
    if path == "-" {
        return Ok(BufReader::new(Box::new(io::stdin())));
    }

    let file_path = Path::new(path);

    if file_path.is_dir() {
        return Err(io::Error::from(ErrorKind::Other));
    }

    let file = File::open(file_path)?;

    Ok(BufReader::new(Box::new(file)))
}

fn line_counts(line: &String, counts: &mut Counts) {
    if line.len() > counts.max_length {
        counts.max_length = line.len();
    }

    counts.chars += line.len();
    counts.bytes += line.bytes().len();
    counts.lines += 1;
    counts.words += line.split_whitespace().count();
}
