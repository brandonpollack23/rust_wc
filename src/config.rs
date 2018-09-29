extern crate clap;

use clap::ArgMatches;

#[derive(Debug)]
pub struct Config {
    print_bytes: bool,
    print_chars: bool,
    print_lines: bool,
    print_words: bool,
    print_max_length: bool,
}

impl Config {
    pub fn new(matches: &ArgMatches) -> Config {
        let config = Config {
            print_bytes: matches.is_present("bytes"),
            print_chars: matches.is_present("chars"),
            print_lines: matches.is_present("lines"),
            print_words: matches.is_present("words"),
            print_max_length: matches.is_present("max_length"),
        };

        if config.print_bytes
            || config.print_chars
            || config.print_lines
            || config.print_words
            || config.print_max_length {
            config
        } else {
            Config {
                print_bytes: true,
                print_chars: false,
                print_lines: true,
                print_words: true,
                print_max_length: false,
            }
        }
    }
}

