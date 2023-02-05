# Webbox

<p align="center"><img src="https://raw.githubusercontent.com/ionutrogojan/webbox/8f35d08f14740561eeb43aa785575d9928cff137/icon/webbox_icon.svg" width="480px"/></p>

### A light weight cli tool written in Rust for its blazingly fast execution while maintaining its safety, blazingly fast 😎

---

### Available on some major platforms:

- [x] [Linux](https://github.com/ionutrogojan/webbox/releases/tag/0.2.0) - v0.2.1
- [x] [Windows](https://github.com/ionutrogojan/webbox/releases/tag/0.2.0) - v0.2.1
- [x] [MacOS](https://github.com/ionutrogojan/webbox/releases/tag/0.1.1) - v0.2.1
- [x] [Source](https://github.com/ionutrogojan/webbox/releases/tag/0.2.0) - v0.2.1

---

## Quick Setup

When you first open the program, there will be no `config.webbox` file. The program creates a new file in the "HOME" directory and exits.

Once the file is created, it will open the default set links, blazingly fast 😎

After the initial execution, use `./webbox -help` to view the available commands

⚠️ - do not manually edit the file, doing so can result in breaking the program. You have been warn.
If the program does happen to break, delete the `config.webbox` file and run the program without commands in order to create a new config file using the internal template.

---

## I can't find my `config.webbox` file! Where is it?

Running `./webbox -config-path`(Linux/MacOS) || `.\webbox -config-path`(Windows) will give you the path to the `config.webbox` file.

```ini
[Linux / MacOS]

You will find the config.webbox file inside the "$HOME/.webbox" directory

[Windows]

You will find config.webbox inside the "%appdata%\Roaming\Webbox" directory

```

---

## Create a shortcut for blazingly fast 😎 access

### *LINUX*
1. open `webbox.desktop`
2. update the paths:
```ini
  Icon=/path_to_icon/webbox_icon.svg
  Exec=/path_to_executable/webbox
```
3. run in the terminal 
```
mv webbox.desktop ~/.local/share/applications
```
4. reboot the system to apply changes

### *Windows*

1. right-click the desktop
2. new > shortcut
3. browse and select the `webbox.exe` file
4. type the name "webbox" and finish
5. right-click the shortcut > properties > change icon
6. confirm the error > browse and select the `webbox.ico` file

### *MacOS*

1. open finder and make a new folder inside of `~/applications`
2. move the `webbox` file inside of it
3. right click the `webbox` file > get info
4. drag the `webbox.icns` over the existing icon
5. right click the `webbox` file > make an alias
6. move the alias to the desktop or dock