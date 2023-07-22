use std::ffi::OsString;
use std::path::{Path, PathBuf};

use serde_derive::{Deserialize, Serialize};
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

/// Bing wallpaper application configuration
#[derive(Serialize, Deserialize)]
pub struct BingWallpaperConfiguration {
    pub(crate) image_dimension_width: u32,
    pub(crate) image_dimension_height: u32,
    pub(crate) target_filename: String,
}


/// `BingWallpaperConfiguration` implements `Default`
impl Default for BingWallpaperConfiguration {
    fn default() -> Self {
        Self {
            image_dimension_height: 1080,
            image_dimension_width: 1920,
            target_filename: "/tmp/bingwallpaper.png".into(),
        }
    }
}

/// Initializes a new configuration file.
///
/// # Arguments
/// * `file_name` - A string that hold the filename where to store initial configuration
///
/// # Examples
///
/// ```
/// use configuration::init_application_configuration_file;
/// init_application_configuration_file("/etc/bingwallpaper.conf");
/// ```
#[allow(deprecated)]
pub fn init_application_configuration_file(file_name: String) {
    // Creates configuration structure
    let mut config: BingWallpaperConfiguration = BingWallpaperConfiguration::default();

    // Tries to detect best values for image dimensions
    let event_loop = EventLoop::new();
    if let Ok(window) = WindowBuilder::new().build(&event_loop) {
        let monitor_size: PhysicalSize<u32> = window.available_monitors().max().unwrap().size();

        config.image_dimension_width = monitor_size.width;
        config.image_dimension_height = monitor_size.height;
    }

    // Target filename ($HOME/.bingwallpaper.png)
    config.target_filename = std::env::home_dir()
        .map(PathBuf::into_os_string)
        .map(OsString::into_string)
        .map(Result::unwrap)
        .map(|mut location| {
            #[cfg(target_os = "macos")]
            location.push_str("/Pictures/bing-wallpaper.png");

            #[cfg(not(target_os = "macos"))]
            location.push_str("/.bingwallpaper.png");

            location
        })
        .unwrap();

    // Creates configuration files
    println!("Write configuration file into {:?}", file_name);
    match confy::store_path(file_name, config) {
        Err(error) => panic!("Can't create configuration file: {:?}", error),
        Ok(_) => println!("Configuration file created"),
    }
}

/// Load application configuration.
/// If filename is empty, the function will try to load the configuration file from other locations.
///
/// # Arguments
/// * `file_name` - A string that hold the filename of the configuration file to load
///
/// # Examples
///
/// ```
/// use configuration::load_application_configuration;
/// let cfg = load_application_configuration("/etc/bingwallpaper.conf");
/// ```
#[allow(deprecated)]
pub fn load_application_configuration(file_name: Option<String>) -> BingWallpaperConfiguration {
    if let Some(value) = file_name {
        confy::load_path(value).unwrap()
    } else {
        let user_configuration_file: String = std::env::home_dir()
            .map(PathBuf::into_os_string)
            .map(OsString::into_string)
            .map(Result::unwrap)
            .map(|mut location| {
                location.push_str("/.bingwallpaper.conf");
                location
            })
            .filter(|location| Path::new(location.as_str()).exists())
            .unwrap_or(String::from("/etc/bingwallpaper.conf"));


        if !Path::new(user_configuration_file.as_str()).exists() {
            panic!("Configuration file does not exist: {:?}", user_configuration_file)
        }

        match confy::load_path(user_configuration_file) {
            Err(error) => panic!("Can't load or create configuration file: {:?}", error),
            Ok(configuration) => configuration,
        }
    }
}
