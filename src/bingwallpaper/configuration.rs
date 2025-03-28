use std::ffi::OsString;
use std::path::{Path, PathBuf};

use serde_derive::{Deserialize, Serialize};
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;

/// Bing wallpaper application configuration
#[derive(Serialize, Deserialize)]
pub struct BingWallpaperConfiguration {
    pub(crate) loop_interval_second: Option<u64>,
    pub(crate) image_dimension_width: u32,
    pub(crate) image_dimension_height: u32,
    pub(crate) target_filename: String,
    pub(crate) text_overlay_position: Option<String>,
    pub(crate) text_overlay_position_offset_x: Option<u32>,
    pub(crate) text_overlay_position_offset_y: Option<u32>,
    pub(crate) exec_apply_wallpaper: Option<String>,
    pub(crate) proxy_url: Option<String>,
}


/// `BingWallpaperConfiguration` implements `Default`
impl Default for BingWallpaperConfiguration {
    fn default() -> Self {
        Self {
            loop_interval_second: 900.into(),
            image_dimension_height: 1080,
            image_dimension_width: 1920,
            target_filename: "/tmp/bingwallpaper.jpg".into(),
            text_overlay_position: None,
            text_overlay_position_offset_x: None,
            text_overlay_position_offset_y: None,
            exec_apply_wallpaper: None,
            proxy_url: None,
        }
    }
}

impl BingWallpaperConfiguration {
    /// Initializes a new configuration file.
    ///
    /// # Arguments
    /// * `file_name` - An optional string that hold the filename where to store initial configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use configuration::init_application_configuration_file;
    /// init_application_configuration_file("/etc/bingwallpaper.conf");
    /// ```
    #[allow(deprecated)]
    pub fn init_file(file_name_option: Option<String>) {
        // Resolves file name to use
        let file_name = BingWallpaperConfiguration::resolve_file_path(file_name_option);
        println!("Creating configuration file {:?}... Please wait!", file_name);

        // Creates configuration structure
        let mut config: BingWallpaperConfiguration = BingWallpaperConfiguration::default();

        // Tries to detect best values for image dimensions
        let event_loop = EventLoop::new();
        let monitor_size: PhysicalSize<u32> = event_loop.unwrap().available_monitors().max().unwrap().size();
        
        config.image_dimension_width = monitor_size.width;
        config.image_dimension_height = monitor_size.height;

        println!("  > Wallpaper dimension: {}x{}", config.image_dimension_width, config.image_dimension_height);

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

        println!("  > Target location: {}", config.target_filename);

        // Creates configuration files
        match confy::store_path(file_name, config) {
            Err(error) => panic!("Can't create configuration file: {:?}", error),
            Ok(_) => println!("Configuration file created!"),
        }
    }

    /// Load application configuration.
    /// If filename is empty, the function will try to load the configuration file from other locations.
    ///
    /// # Arguments
    /// * `file_name_option` - An optional string that hold the filename of the configuration file to load
    ///
    /// # Examples
    ///
    /// ```
    /// use configuration::load_application_configuration;
    /// let cfg = load_application_configuration("/etc/bingwallpaper.conf");
    /// ```
    #[allow(deprecated)]
    pub fn load(file_name_option: Option<String>) -> BingWallpaperConfiguration {
        let file_name = BingWallpaperConfiguration::resolve_file_path(file_name_option);

        if !Path::new(file_name.as_str()).exists() {
            panic!("Configuration file does not exist: {:?}", file_name)
        }

        match confy::load_path(file_name) {
            Err(error) => panic!("Can't load or create configuration file: {:?}", error),
            Ok(configuration) => configuration,
        }
    }

    /// Resolves file path to use.
    #[allow(deprecated)]
    fn resolve_file_path(file_name: Option<String>) -> String {
        match file_name {
            Some(value) => value,
            None => std::env::home_dir()
                .map(PathBuf::into_os_string)
                .map(OsString::into_string)
                .map(Result::unwrap)
                .map(|mut location| {
                    location.push_str("/.bingwallpaper.conf");
                    location
                })
                .filter(|location| Path::new(location.as_str()).exists())
                .unwrap_or(String::from("/etc/bingwallpaper.conf"))
        }
    }
}
