#[cfg(target_os = "windows")]
use std::ffi::CString;
#[cfg(target_os = "macos")]
use std::fs;
#[cfg(target_os = "macos")]
use std::fs::File;
#[cfg(target_os = "macos")]
use std::io::Write;
use std::path::Path;
#[cfg(not(target_os = "windows"))]
use std::process::Command;
use std::time::SystemTime;

use chrono::{DateTime, Utc};

#[cfg(target_os = "windows")]
use winapi::ctypes::c_void;
#[cfg(target_os = "windows")]
use winapi::um::winuser;

use crate::bingwallpaper::{BingAPIClient, BingWallpaperConfiguration};

/// Retrieves from Bing API and applies the wallpaper of the day.
///
/// # Examples
///
/// ```
/// use bingwallpaper::BingWallpaperChanger;
/// let instance = BingWallpaperChanger::new(configuration)
/// instance.try_change();
/// ```
pub struct BingWallpaperChanger {
    configuration: BingWallpaperConfiguration,
    bing_api_client: BingAPIClient,
}

impl BingWallpaperChanger {
    /// Creates a new instance.
    ///
    /// Arguments
    /// * `configuration` - The configuration to use
    ///
    /// # Examples
    ///
    /// ```
    /// use bingwallpaper::BingWallpaperChanger;
    /// let instance = BingWallpaperChanger::new(configuration)
    /// ```
    #[must_use]
    pub fn new(configuration: BingWallpaperConfiguration) -> BingWallpaperChanger {
        let proxy_url = configuration.proxy_url.clone();

        BingWallpaperChanger {
            configuration,
            bing_api_client: BingAPIClient::new(proxy_url),
        }
    }

    /// Tries to download and applies wallpaper of the day.
    pub fn try_change(&self) -> Result<(), String> {
        self.process(true)
    }

    /// Tries to download the wallpaper of the day.
    pub fn try_download(&self) -> Result<(), String> {
        self.process(false)
    }

    /// Do job.
    ///
    /// # Arguments
    /// * `must_change_wallpaper` - `true` to change wallpaper after download
    fn process(&self, must_change_wallpaper: bool) -> Result<(), String> {
        let system_date_as_str = self.get_date_system();
        let wallpaper_date_as_str = self.get_date_current_wallpaper();

        // Checks if current downloaded wallpaper is up to date
        if system_date_as_str == wallpaper_date_as_str {
            return self.change_wallpaper();
        }

        // Retrieves information from Bing API
        let bing_image = match self.bing_api_client.retrieve_latest_image(
            self.configuration.image_dimension_width,
            self.configuration.image_dimension_height) {
            Err(error) => return Err(error),
            Ok(obj) => obj,
        };

        println!("Wallpaper information");
        println!("  - Title    : {}", bing_image.title);
        println!("  - Copyright: {}", bing_image.copyright);
        println!("               {}", bing_image.copyrightlink);

        if wallpaper_date_as_str.contains("00000000") || bing_image.startdate != system_date_as_str {
            // Downloads image
            self.bing_api_client.download_image(&bing_image, &self.configuration.target_filename)?;
        }

        // Change current wallpaper (if requested)
        if must_change_wallpaper {
            return self.change_wallpaper();
        }

        Ok(())
    }

    /// Returns the system date (UTC) as a String following the format "%Y%m%d".
    fn get_date_system(&self) -> String {
        let date_time: DateTime<Utc> = SystemTime::now().into();
        format!("{}", date_time.format("%Y%m%d"))
    }

    /// Returns the date (UTC) as a String following the format "%Y%m%d" of the current wallpaper.
    ///
    /// In case of current wallpaper does not exists, "00000000" will be return.
    fn get_date_current_wallpaper(&self) -> String {
        match Path::new(self.configuration.target_filename.as_str()).metadata() {
            Err(_) => String::from("00000000"),
            Ok(metadata) => match metadata.modified() {
                Err(_) => String::from("00000000"),
                Ok(modified) => {
                    let date_time: DateTime<Utc> = modified.into();
                    format!("{}", date_time.format("%Y%m%d"))
                }
            }
        }
    }

    /// Change wallpaper.
    fn change_wallpaper(&self) -> Result<(), String> {
        #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd"))] {
            self.change_wallpaper_linux();
        }

        #[cfg(target_os = "macos")] {
            self.change_wallpaper_macos();
        }

        #[cfg(target_os = "windows")] {
            self.change_wallpaper_windows();
        }

        Ok(())
    }

    /// Changes the wallpaper with the given picture on Linux.
    #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd"))]
    fn change_wallpaper_linux(&self) {
        Command::new("gsettings")
            .arg("set")
            .arg("org.gnome.desktop.background")
            .arg("picture-uri")
            .arg(&self.configuration.target_filename)
            .spawn()
            .expect("Can't change wallpaper");
    }

    /// Changes the wallpaper with the given picture on MacOS.
    #[cfg(target_os = "macos")]
    fn change_wallpaper_macos(&self) {
        // MacOS does not refresh the screen if the file name of
        // the new wallpaper is the same as the old one.
        let swift_script_path = Path::new("/tmp/bingwallpaper.swift");
        let mut file = File::create(swift_script_path).unwrap();
        // Read more: https://developer.apple.com/documentation/appkit/nsscreen/1388393-screens
        file.write_all("import Cocoa
            do {
                for screen in NSScreen.screens {
                    let url = URL(fileURLWithPath: CommandLine.arguments[1])
                    try NSWorkspace.shared.setDesktopImageURL(url, for: screen, options: [:])
                }
            } catch {
                print(error)
            }".as_bytes()).unwrap();

        let tmp_filename = format!("/tmp/{0}", self.get_date_system());
        fs::copy(self.configuration.target_filename.as_str(), &tmp_filename).unwrap();
        Command::new("swift")
            .arg("/tmp/bingwallpaper.swift")
            .arg(&tmp_filename)
            .spawn()
            .expect("Can't change wallpaper");

        Command::new("swift")
            .arg("/tmp/bingwallpaper.swift")
            .arg(&self.configuration.target_filename)
            .spawn()
            .expect("Can't change wallpaper");

        std::thread::sleep(std::time::Duration::from_secs(2));
        fs::remove_file(&tmp_filename).unwrap();
        fs::remove_file(swift_script_path).unwrap();
    }

    /// Changes the wallpaper with the given picture on Windows.
    #[cfg(target_os = "windows")]
    fn change_wallpaper_windows(&self) {
        let image_path = CString::new(String::from(&self.configuration.target_filename)).unwrap();

        unsafe {
            winuser::SystemParametersInfoA(
                winuser::SPI_SETDESKWALLPAPER,
                0,
                image_path.as_ptr() as *mut c_void,
                winuser::SPIF_UPDATEINIFILE,
            );
        }
    }
}
