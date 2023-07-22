use clap::Parser;

/// Bing wallpaper application arguments
#[derive(Debug, Parser)]
#[clap(name = "bingwallpaper")]
pub struct BingWallpaperArguments {
    #[clap(long = "loop", short = 'l', help = "Keep application running. Looking for new wallpaper every hours")]
    pub(crate) must_loop: bool,

    #[clap(long = "config", short = 'c', help = "Use a custom configuration")]
    pub(crate) config_file: Option<String>,

    #[clap(long = "init-config-file", short = 'i', help = "Initialize a new configuration file")]
    pub(crate) init_config_file: Option<String>,

    #[clap(long = "version", short = 'v', help = "Display application version")]
    pub(crate) show_version: bool,
}

/// Parses application arguments.
///
/// # Examples
///
/// ```
/// use arguments::parse_application_arguments;
/// let args = parse_application_arguments();
/// ```
pub fn parse_application_arguments() -> BingWallpaperArguments {
    return BingWallpaperArguments::parse();
}
