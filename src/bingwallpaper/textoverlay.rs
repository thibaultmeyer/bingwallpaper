use crate::bingwallpaper::BingWallpaperConfiguration;
use ab_glyph::{FontRef, PxScale};
use image::{ImageReader, Rgb, RgbImage};
use imageproc::drawing::draw_text;

/// Text overlay
pub struct TextOverlay {}

impl TextOverlay {
    /// Applies text overlay.
    ///
    /// # Arguments
    /// * `configuration - The Bing Wallpaper configuration to use
    /// * `line1` - First line of text
    /// * `line2` - Second line of text
    ///
    /// # Examples
    ///
    /// ```
    /// use textoverlay::TextOverlay;
    ///
    /// TextOverlay::apply_overlay(configuration, "Title", "Description")
    /// ```
    pub fn apply_overlay(configuration: &BingWallpaperConfiguration, line1: String, line2: String) {
        if configuration.text_overlay_position.is_none() {
            return;
        }

        let font = FontRef::try_from_slice(include_bytes!("../../res/font/Ubuntu-Regular.ttf")).unwrap();
        let font_scale = PxScale::from(30.0);
        let (mut pos_x, mut pos_y) = match configuration.text_overlay_position.clone().unwrap().to_uppercase().as_str() {
            "TOP_LEFT" => (60i32, 60i32),
            "BOTTOM_LEFT" => (60i32,
                              configuration.image_dimension_height as i32 - (font_scale.x as i32 * 2) - 60 - 5),
            "TOP_RIGHT" => (configuration.image_dimension_width as i32 - (12i32 * line2.len() as i32) - 55,
                            60i32),
            "BOTTOM_RIGHT" => (configuration.image_dimension_width as i32 - (12i32 * line2.len() as i32) - 55,
                               configuration.image_dimension_height as i32 - (font_scale.x as i32 * 2) - 60 - 5),
            _ => (60i32, 60i32),
        };

        if configuration.text_overlay_position_offset_x.is_some() {
            pos_x += configuration.text_overlay_position_offset_x.unwrap() as i32
        }
        if configuration.text_overlay_position_offset_y.is_some() {
            pos_y += configuration.text_overlay_position_offset_y.unwrap() as i32;
        }

        let image = ImageReader::open(&configuration.target_filename).unwrap().decode().unwrap();
        let image = RgbImage::from(image);
        let image = draw_text(&image, Rgb([255u8, 255u8, 255u8]), pos_x, pos_y, font_scale, &font, &line1);
        let image = draw_text(&image, Rgb([255u8, 255u8, 255u8]), pos_x, pos_y + font_scale.x as i32 + 5, font_scale, &font, &line2);
        image.save(&configuration.target_filename).unwrap();
    }
}
