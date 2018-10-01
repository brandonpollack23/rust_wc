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

    let files = matches.values_of("FILES").unwrap();

    let results = files.map(run_coreutil_wc);

    let config = Config::new(&matches);

    results.for_each(|result| -> () {
        println!("{}", result.ok().unwrap().create_result_string(&config))
    });
}
