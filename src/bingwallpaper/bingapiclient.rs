use std::fs::File;
use std::io::Cursor;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_derive::Deserialize;

/// Bing API HTTP client.
pub struct BingAPIClient {
    api_endpoint: String,
}

/// Bing API "Images Archives": root object.
#[derive(Deserialize)]
pub struct BingAPIImagesArchive {
    /// Images founds on the images archive.
    pub images: Vec<BingAPIImagesArchiveImage>,
}

/// Bing API "Images Archives": sub object "Image".
#[derive(Deserialize)]
pub struct BingAPIImagesArchiveImage {
    /// URL of the image without endpoint information (ie : /th?id=OHR...).
    pub url: String,

    /// Title of the image.
    pub title: String,

    /// Copyright information.
    pub copyright: String,

    /// Link (URL) to the copyright information page.
    pub copyrightlink: String,

    /// Date on which the image is proposed as wallpaper of the day.
    pub startdate: String,
}

impl BingAPIClient {
    /// Creates a new instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use bingwallpaper::BingAPIClient;
    /// let instance = BingAPIClient::new();
    /// ```
    #[must_use]
    pub fn new() -> BingAPIClient {
        BingAPIClient {
            api_endpoint: String::from("https://www.bing.com")
        }
    }

    /// Retrieves the latest image from the images archive.
    ///
    /// # Arguments
    /// * `img_dimension_width` - Requested image dimension "width"
    /// * `img_dimension_height` - Requested image dimension "height"
    ///
    /// # Examples
    ///
    /// ```
    /// use bingwallpaper::BingAPIClient;
    /// let instance = BingAPIClient::new();
    ///
    /// let img = instance.retrieve_latest_image(image, 1920, 1080);
    /// ```
    pub fn retrieve_latest_image(&self, img_dimension_width: u32, img_dimension_height: u32) -> Result<BingAPIImagesArchiveImage, String> {

        // Build URI to call
        let time_ms = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let image_archive_api_uri: String = format!(
            "{0}/HPImageArchive.aspx?format=js&idx=0&n=1&nc={1}&uhd=1&uhdwidth={2}&uhdheight={3}",
            self.api_endpoint,
            time_ms,
            img_dimension_width,
            img_dimension_height);

        // Call Bing API
        let http_response = match reqwest::blocking::get(image_archive_api_uri) {
            Err(error) => return Err(format!("Can't fetch Bing API: {:?}", error)),
            Ok(http_response) => http_response,
        };

        // Parses JSON document
        let image_archive = match http_response.json::<BingAPIImagesArchive>() {
            Err(error) => return Err(format!("Can't parse JSON document: {:?}", error)),
            Ok(bing_api_response) => bing_api_response,
        };

        // Returns the latest image
        let image = image_archive.images.get(0).unwrap();

        Ok(BingAPIImagesArchiveImage { // TODO: possible to implement ".clone()" or ".copy" method ????
            url: image.url.clone(),
            title: image.title.clone(),
            copyright: image.copyright.clone(),
            copyrightlink: image.copyrightlink.clone(),
            startdate: image.startdate.clone(),
        })
    }

    /// Downloads image.
    ///
    /// # Arguments
    /// * `image` - The image to download
    /// * `target` - The location where to save image
    ///
    /// # Examples
    ///
    /// ```
    /// use bingwallpaper::BingAPIClient;
    /// let instance = BingAPIClient::new();
    ///
    /// instance.download_image(image, "/tmp/out.png");
    /// ```
    pub fn download_image(&self, image: &BingAPIImagesArchiveImage, target: &String) -> Result<(), String> {
        let image_content_uri: String = format!("{0}{1}", self.api_endpoint, image.url);
        let image_response = reqwest::blocking::get(image_content_uri).unwrap();
        let mut output_file = File::create(target).unwrap();
        let mut image_content = Cursor::new(image_response.bytes().unwrap());

        if let Err(error) = std::io::copy(&mut image_content, &mut output_file) {
            Err(format!("Can't store file: {:?}", error))
        } else {
            Ok(())
        }
    }
}
