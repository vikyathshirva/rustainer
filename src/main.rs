mod cli;
mod errors;
mod container;
mod config;
use errors::exit_with_retcode;
use dotenv::dotenv;
use std::process::exit;

#[macro_use] extern crate scan_fmt;

fn main() {
    dotenv().ok();
    match cli::parse_args(){
        Ok(args) => {
            log::info!("{:?}", args);
            exit_with_retcode(container::Container::start(args))
        },
        Err(e) => {
            log::error!("Error while passing arguments:\n\t{}", e);
            exit(e.get_retcode());
        }
    };
}
