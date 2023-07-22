use std::{process, thread};
use std::thread::{JoinHandle, sleep};
use std::time::Duration;

use crate::bingwallpaper::{arguments, configuration};
use crate::bingwallpaper::wallpaper_changer::change_wallpaper;

mod bingwallpaper;

/// Application entry point
fn main() {

    // Parse application arguments
    let args = arguments::parse_application_arguments();

    // If requested, initialize a new configuration file
    if args.init_config_file.is_some() {
        configuration::init_application_configuration_file(args.init_config_file.unwrap());
        process::exit(0);
    }

    // Load configuration
    let config = configuration::load_application_configuration(args.config_file);

    // Run
    if args.must_loop == true {
        let thread_handle: JoinHandle<()> = thread::Builder::new().name("bingwallpaper".to_string()).spawn(move || {
            loop {
                change_wallpaper(&config);
                sleep(Duration::from_secs(3600));
            }
        }).unwrap();

        thread_handle.join().unwrap();
    } else {
        change_wallpaper(&config);
    }
}
