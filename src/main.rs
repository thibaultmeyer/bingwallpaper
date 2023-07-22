use std::process;

use crate::bingwallpaper::{arguments, configuration};

mod bingwallpaper;

/// Application entry point
fn main() {

    // Parse application arguments
    let args = arguments::parse_application_arguments();

    // If requested, create a new configuration file
    if args.init_config_file.is_some() {
        configuration::init_application_configuration_file(args.init_config_file.unwrap());
        process::exit(0);
    }

    // Load configuration
    let config = configuration::load_application_configuration(args.config_file);

    // Run
    if args.run_as_daemon == true {
        println!("Run as daemon");
    } else {
        println!("Run single {:?}", config.target_filename);
    }
}
