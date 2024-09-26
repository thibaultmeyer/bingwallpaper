use chrono::{DateTime, Utc};
#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd"))]
use std::env;
#[cfg(target_os = "windows")]
use std::ffi::CString;
#[cfg(target_os = "macos")]
use std::fs;
#[cfg(target_os = "macos")]
use std::fs::File;
#[cfg(target_os = "macos")]
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::time::SystemTime;
#[cfg(target_os = "windows")]
use winapi::ctypes::c_void;
#[cfg(target_os = "windows")]
use winapi::um::winuser;
#[cfg(target_os = "windows")]
use winvd::{get_desktop_count, get_desktops};
#[cfg(target_os = "windows")]
use winver::WindowsVersion;

use crate::bingwallpaper::{BingAPIClient, BingWallpaperConfiguration, TextOverlay};

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
    /// * `configuration` - The Bing Wallpaper configuration to use
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
        println!("  - Title    : {}", &bing_image.title);
        println!("  - Copyright: {}", &bing_image.copyright);
        println!("               {}", &bing_image.copyrightlink);

        if wallpaper_date_as_str.contains("00000000") || bing_image.startdate != system_date_as_str {
            // Downloads image
            self.bing_api_client.download_image(&bing_image, &self.configuration.target_filename)?;
        }

        // Overlay
        TextOverlay::apply_overlay(&self.configuration, bing_image.title, bing_image.copyright);

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
    /// In case of current wallpaper does not exist, "00000000" will be return.
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
        if self.configuration.exec_apply_wallpaper.is_some() {
            self.exec_apply_wallpaper();
        } else {
            #[cfg(any(
                target_os = "linux",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd"
            ))] {
                self.change_wallpaper_linux();
            }

            #[cfg(target_os = "macos")] {
                self.change_wallpaper_macos();
            }

            #[cfg(target_os = "windows")] {
                self.change_wallpaper_windows();
            }
        }

        Ok(())
    }

    /// Changes the wallpaper by executing custom command
    fn exec_apply_wallpaper(&self) {
        // Replaces all variables
        let mut cmd_as_str = self.configuration.exec_apply_wallpaper.clone().unwrap();
        cmd_as_str = str::replace(&cmd_as_str, "{target_filename}", &self.configuration.target_filename);
        cmd_as_str = str::replace(&cmd_as_str, "{image_dimension_width}", &self.configuration.image_dimension_width.to_string());
        cmd_as_str = str::replace(&cmd_as_str, "{image_dimension_height}", &self.configuration.image_dimension_height.to_string());

        // Prepares command to run
        if !cmd_as_str.is_empty() {
            let mut cmd_to_run;
            let cmd_tokens = cmd_as_str.split_whitespace();

            if cmd_as_str.len() == 1 {
                cmd_to_run = Command::new(cmd_as_str);
            } else {
                let mut it = cmd_tokens.into_iter().peekable();

                cmd_to_run = Command::new(it.next().unwrap());
                while it.peek().is_some() {
                    cmd_to_run.arg(it.next().unwrap());
                }
            }

            // Run command
            let mut child = cmd_to_run
                .spawn()
                .expect("Can't change wallpaper");
            child.wait().expect("Can't wait for child process");
        }
    }

    /// Changes the wallpaper with the given picture on Linux.
    #[cfg(any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    fn change_wallpaper_linux(&self) {
        let session = env::var("DESKTOP_SESSION").unwrap();

        if session.eq("cinnamon") {
            // Cinnamon
            let mut child = Command::new("gsettings")
                .arg("set")
                .arg("org.cinnamon.desktop.background")
                .arg("picture-uri")
                .arg(format!("file://{}", &self.configuration.target_filename))
                .spawn()
                .expect("Can't change wallpaper");
            child.wait().expect("Can't wait for child process");
        } else {
            // Gnome
            let mut child = Command::new("gsettings")
                .arg("set")
                .arg("org.gnome.desktop.background")
                .arg("picture-uri")
                .arg(&self.configuration.target_filename)
                .spawn()
                .expect("Can't change wallpaper");
            child.wait().expect("Can't wait for child process");

            let mut child = Command::new("gsettings")
                .arg("set")
                .arg("org.gnome.desktop.background")
                .arg("picture-uri-dark")
                .arg(&self.configuration.target_filename)
                .spawn()
                .expect("Can't change wallpaper");
            child.wait().expect("Can't wait for child process");
        };
    }

    /// Changes the wallpaper with the given picture on MacOS.
    #[cfg(target_os = "macos")]
    fn change_wallpaper_macos(&self) {
        // TODO: copy file with unique filename (hidden file) and apply it!
        // Writes script SWIFT used to change wallpaper into temporary location
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

        // MacOS does not refresh the screen if the file name of
        // the new wallpaper is the same as the old one.
        let target_filename_as_path = Path::new(&self.configuration.target_filename);
        let tmp_filename_prefix = format!(
            "{0}/._{1}_",
            target_filename_as_path.parent().unwrap().to_str().unwrap(),
            target_filename_as_path.file_name().unwrap().to_str().unwrap());

        // Delete old temporary wallpapers
        for dir_entry in fs::read_dir(target_filename_as_path.parent().unwrap()).unwrap() {
            let path = dir_entry.unwrap().path();
            let path_as_str = path.to_str().unwrap();
            if path_as_str.starts_with(&tmp_filename_prefix) {
                fs::remove_file(path).unwrap();
            }
        }

        // Apply new temporary wallpaper
        let tmp_filename = format!("{0}{1}", tmp_filename_prefix, self.get_date_system());
        fs::copy(self.configuration.target_filename.as_str(), &tmp_filename).unwrap();
        let mut child = Command::new("swift")
            .arg("/tmp/bingwallpaper.swift")
            .arg(&tmp_filename)
            .spawn()
            .expect("Can't change wallpaper");
        child.wait().expect("Can't wait for child process");
        std::thread::sleep(std::time::Duration::from_millis(250));

        fs::remove_file(swift_script_path).unwrap();
    }

    /// Changes the wallpaper with the given picture on Windows.
    #[cfg(target_os = "windows")]
    fn change_wallpaper_windows(&self) {
        let win_version = WindowsVersion::detect().unwrap();

        if win_version >= WindowsVersion::new(10, 0, 22621) && get_desktop_count().unwrap() > 1 {
            if let Err(error) = self.change_wallpaper_windows_virtualdesktop() {
                println!("Something goes wrong with Virtual Desktop API. Fallback to legacy Windows API\n{:?}", error);
                self.change_wallpaper_windows_winuser();
            }
        } else {
            self.change_wallpaper_windows_winuser();
        }
    }

    /// Changes the wallpaper with the given picture on Windows using the Virtual Desktop API.
    #[cfg(target_os = "windows")]
    fn change_wallpaper_windows_virtualdesktop(&self) -> Result<(), String> {
        if let Ok(detected_desktops) = get_desktops() {
            for desktop in detected_desktops {
                if let Err(error) = desktop.set_wallpaper(&self.configuration.target_filename) {
                    return Err(format!(
                        "Can't change Virtual Desktop wallpaper for #{:?}\n{:?}",
                        desktop.get_id().unwrap(),
                        error));
                }
            }

            Ok(())
        } else {
            Err("Can't detect Virtual Desktop ".to_string())
        }
    }

    /// Changes the wallpaper with the given picture on Windows using the legacy Windows API.
    #[cfg(target_os = "windows")]
    fn change_wallpaper_windows_winuser(&self) {
        let image_path = CString::new(String::from(&self.configuration.target_filename)).unwrap();
        unsafe {
            winuser::SystemParametersInfoA(
                winuser::SPI_SETDESKWALLPAPER,
                0,
                image_path.as_ptr() as *mut c_void,
                winuser::SPIF_UPDATEINIFILE);
        }
    }
}
