use std::ffi::OsString;
use std::path::{Path, PathBuf};

use serde_derive::{Deserialize, Serialize};

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
pub fn init_application_configuration_file(file_name: String) {
    let config: BingWallpaperConfiguration = BingWallpaperConfiguration::default();
    match confy::store_path(file_name, config) {
        Err(error) => panic!("Can't create configuration file: {:?}", error),
        _ => {}
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
    return if file_name.is_some() {
        confy::load_path(file_name.unwrap()).unwrap()
    } else {
        let user_configuration_file: String = std::env::home_dir()
            .map(PathBuf::into_os_string)
            .map(OsString::into_string)
            .map(Result::unwrap)
            .map(|mut location| {
                location.push_str("/.bingwallpaper.conf");
                return location;
            })
            .filter(|location| Path::new(location.as_str()).exists())
            .or(Some(String::from("/etc/bingwallpaper.conf")))
            .unwrap();


        if Path::new(user_configuration_file.as_str()).exists() == false {
            panic!("Configuration file does not exist: {:?}", user_configuration_file)
        }

        return match confy::load_path(user_configuration_file) {
            Err(error) => panic!("Can't load or create configuration file: {:?}", error),
            Ok(configuration) => configuration,
        };
    };
}
