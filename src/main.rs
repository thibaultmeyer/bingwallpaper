use std::{env, process, thread};
use std::thread::{JoinHandle, sleep};
use std::time::Duration;

use clap::Parser;

use bingwallpaper::BingWallpaperArguments;
use bingwallpaper::BingWallpaperChanger;
use bingwallpaper::BingWallpaperConfiguration;
#[cfg(target_os = "windows")]
use winapi::um::wincon::GetConsoleWindow;
#[cfg(target_os = "windows")]
use winapi::um::winuser::{ShowWindow, SW_HIDE};

mod bingwallpaper;

/// Application entry point.
fn main() {
    // Parse application arguments
    let args = BingWallpaperArguments::parse();

    // If requested, display version
    if args.show_version {
        println!(
            "{} version {} ({}/{})",
            &env!("CARGO_PKG_NAME"),
            &env!("CARGO_PKG_VERSION"),
            env::consts::OS,
            env::consts::ARCH);

        process::exit(0);
    }

    // If request, hide console
    #[cfg(target_os = "windows")]
    if args.nowindow {
        let window = unsafe { GetConsoleWindow() };
        if !window.is_null() {
            unsafe {
                ShowWindow(window, SW_HIDE);
            }
        }
    }

    // If requested, initialize a new configuration file
    if args.init_config_file.is_some() {
        BingWallpaperConfiguration::init_file(args.init_config_file);
        process::exit(0);
    }

    // Load configuration file
    let config = BingWallpaperConfiguration::load(args.config_file);
    let sleep_duration_sec = config.loop_interval_second;

    // Creates BingWallpaperChanger instance
    let bing_wallpaper_changer = BingWallpaperChanger::new(config);

    // Run
    if args.must_loop {
        let thread_handle: JoinHandle<()> = thread::Builder::new().name("bingwallpaper".to_string()).spawn(move || {
            loop {
                if args.download_only {
                    if let Err(error) = bing_wallpaper_changer.try_download() {
                        println!("Can't download wallpaper: {:?}", error);
                    }
                } else if let Err(error) = bing_wallpaper_changer.try_change() {
                    println!("Can't change wallpaper: {:?}", error);
                }

                sleep(Duration::from_secs(sleep_duration_sec.unwrap_or(900)));
            }
        }).unwrap();

        thread_handle.join().unwrap();
    } else if args.download_only {
        if let Err(error) = bing_wallpaper_changer.try_download() {
            panic!("Can't download wallpaper: {:?}", error);
        }
    } else if let Err(error) = bing_wallpaper_changer.try_change() {
        panic!("Can't change wallpaper: {:?}", error);
    }
}
