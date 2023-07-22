use std::env;
#[cfg(target_os = "windows")]
use std::ffi::CString;
use std::fs::File;
use std::io::Cursor;
#[cfg(not(target_os = "windows"))]
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use reqwest::blocking::Response;
use serde_derive::Deserialize;
#[cfg(target_os = "windows")]
use winapi::ctypes::c_void;
#[cfg(target_os = "windows")]
use winapi::um::winuser;

use crate::bingwallpaper::configuration::BingWallpaperConfiguration;

/// Bing API Response, root object.
#[derive(Deserialize)]
struct BingAPIResponse {
    images: Vec<BingAPIImage>,
}

/// Bing API Response, sub object "Image"
#[derive(Deserialize)]
struct BingAPIImage {
    url: String,
    title: String,
    copyright: String,
    copyrightlink: String,
}

/// Changes the wallpaper with the picture of the day.
///
/// # Arguments
/// * `configuration` - The application configuration
///
/// # Examples
///
/// ```
/// use wallpaper_changer::change_wallpaper;
/// change_wallpaper(configuration);
/// ```
#[allow(unreachable_code)]
pub fn change_wallpaper(configuration: &BingWallpaperConfiguration) {

    // Retrieves JSON document from Bing API
    let bing_api_endpoint: String = String::from("https://www.bing.com");
    let time_ms: u128 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let image_archive_api_uri: String = format!(
        "{0}/HPImageArchive.aspx?format=js&idx=0&n=1&nc={1}&uhd=1&uhdwidth={2}&uhdheight={3}",
        bing_api_endpoint,
        time_ms,
        configuration.image_dimension_width,
        configuration.image_dimension_height);

    // Call Bing API
    let http_response: Response = match reqwest::blocking::get(image_archive_api_uri) {
        Err(error) => panic!("Can't fetch Bing API: {:?}", error),
        Ok(http_response) => http_response,
    };

    let bing_api_response: BingAPIResponse = match http_response.json::<BingAPIResponse>() {
        Err(error) => panic!("Can't parse JSON: {:?}", error),
        Ok(bing_api_response) => bing_api_response,
    };

    println!("Wallpaper information");
    println!("  - Title    : {}", bing_api_response.images[0].title);
    println!("  - Copyright: {}", bing_api_response.images[0].copyright);
    println!("               {}", bing_api_response.images[0].copyrightlink);

    // Download picture
    let image_content_uri: String = format!("{0}{1}", bing_api_endpoint, bing_api_response.images[0].url);
    let image_response = reqwest::blocking::get(image_content_uri).unwrap();
    let mut output_file = File::create(&configuration.target_filename).unwrap();
    let mut image_content = Cursor::new(image_response.bytes().unwrap());

    if let Err(error) = std::io::copy(&mut image_content, &mut output_file) {
        panic!("Can't save wallpaper: {:?}", error);
    }

    // Call the right method to change wallpaper
    #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd"))] {
        change_wallpaper_linux(&configuration.target_filename);
        return;
    }

    #[cfg(target_os = "macos")] {
        change_wallpaper_macos(&configuration.target_filename);
        return;
    }

    #[cfg(target_os = "windows")] {
        change_wallpaper_windows(&configuration.target_filename);
        return;
    }

    // Error
    panic!("Your operating system is not handled: {}", env::consts::OS)
}

/// Changes the wallpaper with the given picture on Linux.
///
/// # Arguments
/// * `file_name` - The picture file name
#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd"))]
fn change_wallpaper_linux(_file_name: &String) {
    panic!("Linux is not yet handled") // TODO: rename _file_name -> file_name
}

/// Changes the wallpaper with the given picture on MacOS.
///
/// # Arguments
/// * `file_name` - The picture file name
#[cfg(target_os = "macos")]
fn change_wallpaper_macos(file_name: &String) {
    Command::new("osascript")
        .arg("-e")
        .arg(format!("tell application \"Finder\" to set desktop picture to POSIX file \"{0}\"", file_name))
        .spawn()
        .expect("Can't change wallpaper");
}

/// Changes the wallpaper with the given picture on Windows.
///
/// # Arguments
/// * `file_name` - The picture file name
#[cfg(target_os = "windows")]
fn change_wallpaper_windows(file_name: &String) {
    let image_path = CString::new(String::from(file_name)).unwrap();

    unsafe {
        winuser::SystemParametersInfoA(
            winuser::SPI_SETDESKWALLPAPER,
            0,
            image_path.as_ptr() as *mut c_void,
            winuser::SPIF_UPDATEINIFILE,
        );
    }
}
