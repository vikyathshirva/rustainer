mod cli;
mod errors;
use errors::exit_with_retcode;
use dotenv::dotenv;
use std::process::exit;


fn main() {
    dotenv().ok();
    match cli::parse_args(){
        Ok(args) => {
            log::info!("{:?}", args);
            exit_with_retcode(Ok(()))
        },
        Err(e) => {
            log::error!("Error while passing arguments:\n\t{}", e);
            exit(e.get_retcode());
        }
    };
}
