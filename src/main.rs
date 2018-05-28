extern crate sexuality_def_bot;

use sexuality_def_bot::configuration;

use std::process;

fn main() {
    let config = configuration::get().unwrap_or_else(|e| {
        eprintln!("Problem getting configuration data: {}", e);
        process::exit(1);
    });
    sexuality_def_bot::run(&config).unwrap_or_else(|e| {
        eprintln!("Problem running bot: {}", e);
        process::exit(1);
    });
}
