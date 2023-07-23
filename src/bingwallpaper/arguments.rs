use clap::Parser;

/// Bing wallpaper application arguments
#[derive(Debug, Parser)]
#[clap(name = "bingwallpaper")]
pub struct BingWallpaperArguments {
    /// If `true`, the application must continue to looking for new version of the image to use as wallpaper.
    #[clap(long = "loop", short = 'l', help = "Keep application running. Looking for new wallpaper every hours")]
    pub(crate) must_loop: bool,

    /// Custom configuration file to use.
    #[clap(long = "config", short = 'c', help = "Use a custom configuration")]
    pub(crate) config_file: Option<String>,

    /// Custom configuration file to initialize.
    #[clap(long = "init-config-file", short = 'i', help = "Initialize a new configuration file")]
    pub(crate) init_config_file: Option<String>,

    /// If `true`, the wallpaper image must be downloaded but not applied.
    #[clap(long = "download-only", short = 'o', help = "Download wallpaper, but dont try to change it automatically")]
    pub(crate) download_only: bool,

    /// If `true`, the application must show "version" information and exit.
    #[clap(long = "version", short = 'v', help = "Display application version")]
    pub(crate) show_version: bool,
}

