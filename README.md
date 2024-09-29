# bingwallpaper

[![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/thibaultmeyer/bingwallpaper/blob/master/LICENSE)
[![Repository release](https://img.shields.io/github/v/release/thibaultmeyer/bingwallpaper?logo=github)](https://github.com/thibaultmeyer/bingwallpaper/releases)
[![Repository size](https://img.shields.io/github/repo-size/thibaultmeyer/bingwallpaper.svg?logo=git)](https://github.com/thibaultmeyer/bingwallpaper)

[![Java CI](https://img.shields.io/github/actions/workflow/status/thibaultmeyer/bingwallpaper/build.yml?logo=github&color=%231da868&branch=main)](https://github.com/thibaultmeyer/bingwallpaper/actions/workflows/build.yml)


Bing wallpaper of the day
*****


## Build & install from sources
To compile and install this project, you must ensure that Rust and adequate toolchain
are being correctly installed.

```shell
#> cargo build --release
#> strip ./target/release/bingwallpaper

#> cp ./target/release/bingwallpaper /opt/bingwallpaper
#> chmod a+x /opt/bingwallpaper
#> ln -s /opt/bingwallpaper /usr/local/bin/bingwallpaper
```



## Usage

```text
#> bingwallpaper --help
Bing wallpaper application arguments

Usage: bingwallpaper [OPTIONS]

Options:
  -c, --config <CONFIG_FILE>
          Use a custom configuration
  -i, --init-config-file <INIT_CONFIG_FILE>
          Initialize a new configuration file
  -o, --download-only
          Download wallpaper, but dont try to change it automatically
  -l, --loop
          Keep application running. Looking for new wallpaper every 900 seconds (15 minutes).
          You can override value with `loop_interval_second` in your configuration file.
  -w, --nowindow
          Don't display console when not run from a CLI (Windows Only)
  -v, --version
          Display application version
  -h, --help
          Print help
```



## First run

First of all, you need to generate the configuration file. To do this, simply run
the following command:


**Linux or MacOS**
```shell
#> bingwallpaper --init-config-file ~/.bingwallpaper.conf
```


**Windows**
```shell
#> bingwallpaper --init-config-file C:/Users/<USERNAME>/.bingwallpaper.conf
```

The application will try to detect the right settings for your operating system and
screen resolution. It is still advisable to check that the information is correct
before continuing.

Once the configuration file has been checked. Run the following command to change the
background.

```shell
#> bingwallpaper [--loop] [--config <alternative cfg file>]
```



## Configuration file

* `loop_interval_second` The interval in seconds between two wallpaper update attempts. Default value is `900`
* `image_dimension_width` The "width" dimension of the wallpaper
* `image_dimension_height` The "height" dimension of the wallpaper
* `target_filename` The location where is stored the wallpaper. File extension must be `.jpg`
* `text_overlay_position` (OPTIONAL) Add text overlay containing information about the picture. Value must be 
   surrounded with simple quote. Accepted values are: `BOTTOM_LEFT`, `BOTTOM_RIGHT`, `TOP_LEFT`, `TOP_RIGHT`
* `text_overlay_position_offset_x` (OPTIONAL) Applies an offset on the X-axis of the text overlay.
* `text_overlay_position_offset_y` (OPTIONAL) Applies an offset on the Y-axis of the text overlay.
* `exec_apply_wallpaper` (OPTIONAL) Command to execute for applying wallpaper, the
   string accept following variables: `image_dimension_width`, `image_dimension_height`, 
   and `target_filename`
* `proxy_url` (OPTIONAL) The proxy URL (ie: http://127.0.0.1:8080)

**Note:** You can use "#" to comment a line



## Change wallpaper automatically

Before attempting to automate the process (see below), it is advisable to run it
manually at least once to check that everything is working correctly.


### Linux

On Linux, there are several possibilities. As I can't list them all, you'll have to
choose the one that seems best suited to you (crontab, gnome start at launch, etc.).

```shell
#> crontab -e
```

```
0 * * * * /usr/local/bin/bingwallpaper >> /tmp/bingwallpaper.log 2>&1
```


### MacOS
**~/Library/LaunchAgents/com.github.thibaultmeyer.bingwallpaper.plist**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
    <dict>
        <key>Label</key>
        <string>com.github.thibaultmeyer.bingwallpaper</string>
        <key>ProgramArguments</key>
        <array>
            <string>/usr/local/bin/bingwallpaper</string>
            <string>--loop</string>
        </array>
        <key>RunAtLoad</key>
        <true/>
        <key>KeepAlive</key>
        <true/>
        <key>StandardOutPath</key>
        <string>/tmp/bingwallpaper.log</string>
        <key>StandardErrorPath</key>
        <string>/tmp/bingwallpaper.log</string>
    </dict>
</plist>
```



## Known issues / limitations

* [LINUX] Only Gnome and Cinnamon are natively supported, to use this application with 
  another Desktop Environment, you have to use `exec_apply_wallpaper` option in the
  configuration file
* [MAC] Sometime, "Dock" refuse to refresh wallpaper when you are using an application 
  in fullscreen when bingwallpaper try to update wallpaper
* [WINDOWS] Parameter `--nowindow` (or `-w`) not working if you use Terminal as default
  console. Terminal don't honor Windows API and don't allow application to detach from
  terminal. As workaround, you could create a shortcut and configure it to enable the
  option "legacy console"



## License
This project is released under terms of the [MIT license](https://github.com/thibaultmeyer/bingwallpaper/blob/master/LICENSE).



## Third party
Logo is from [Flaticon](https://www.flaticon.com) with
free usage for personal and commercial purpose.
<a href="https://www.flaticon.com/free-icons/gallery" title="gallery icons">Gallery icons created by logisstudio - Flaticon</a>
