# bingwallpaper

[![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/thibaultmeyer/bingwallpaper/blob/master/LICENSE)
[![Repository release](https://img.shields.io/github/v/release/thibaultmeyer/bingwallpaper?logo=github)](https://github.com/thibaultmeyer/bingwallpaper/releases)
[![Repository size](https://img.shields.io/github/repo-size/thibaultmeyer/bingwallpaper.svg?logo=git)](https://github.com/thibaultmeyer/bingwallpaper)

[![Java CI](https://img.shields.io/github/actions/workflow/status/thibaultmeyer/bingwallpaper/build.yml?logo=github&color=%231da868&branch=main)](https://github.com/thibaultmeyer/bingwallpaper/actions/workflows/build.yml)


Bing wallpaper of the day
*****


## Build & install from sources
To compile and install this project, you must ensure that Rust and adequate toolchain are being correctly installed.

```shell
#> cargo build --release
#> strip ./target/release/bingwallpaper

#> cp ./target/release/bingwallpaper /opt/bingwallpaper
#> chmod a+x /opt/bingwallpaper
#> ln -s /opt/bingwallpaper /usr/local/bin/bingwallpaper
```



## Usage

```shell
#> bingwallpaper --help
Bing wallpaper application arguments

Usage: bingwallpaper [OPTIONS]

Options:
  -l, --loop
          Keep application running. Looking for new wallpaper every hours
  -c, --config <CONFIG_FILE>
          Use a custom configuration
  -i, --init-config-file <INIT_CONFIG_FILE>
          Initialize a new configuration file
  -o, --download-only
          Download wallpaper, but dont try to change it automatically
  -v, --version
          Display application version
  -h, --help
          Print help

```



## First run

First of all, you need to generate the configuration file. To do this, simply run the following command:

**Linux or MacOS**
```shell
#> bingwallpaper --init-config-file ~/.bingwallpaper.conf
```

**Windows**
```shell
#> bingwallpaper --init-config-file C:/Users/<USERNAME>/.bingwallpaper.conf
```

The application will try to detect the right settings for your operating system and screen resolution. It is still advisable to check that the information is correct before continuing.

Once the configuration file has been checked. Run the following command to change the background.

```shell
#> bingwallpaper [--loop] [--config <alternative cfg file>]
```



## Change wallpaper automatically

Before attempting to automate the process (see below), it is advisable to run it manually at least once to check that everything is working correctly.

### Linux

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



## License
This project is released under terms of the [MIT license](https://github.com/thibaultmeyer/bingwallpaper/blob/master/LICENSE).
