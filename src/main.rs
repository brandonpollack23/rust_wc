#[macro_use]
extern crate clap;

mod config;
mod word_count;

use config::Config;
use word_count::*;

// TODO error handling
// TODO files0-from

fn main() {
    let yaml = load_yaml!("resources/cli.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();

    let result = run_coreutil_wc("testfile").unwrap();

    let config = Config::new(&matches);
    println!("{}", result.create_result_string(&config))
}
