use config::Config;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::ErrorKind;
use std::path::Path;
use std::io::BufRead;
use std::io::Read;

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
        String::from("")
    }
}

// TODO make execute on vec
pub fn run_coreutil_wc(path: &str) -> io::Result<Counts> {
    let mut file_reader = open(path)?;

    let mut test_string = String::new();
    file_reader.read_line(&mut test_string);
    println!("file contents: {}", test_string);

    Ok(Counts::zeros())
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

