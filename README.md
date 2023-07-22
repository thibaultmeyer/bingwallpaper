# bingwallpaper

[![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/thibaultmeyer/bingwallpaper/blob/master/LICENSE)
[![Repository release](https://img.shields.io/github/v/release/thibaultmeyer/bingwallpaper?logo=github)](https://github.com/voidframework/voidframework/releases)

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
**com.github.thibaultmeyer.bingwallpaper.plist**
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
