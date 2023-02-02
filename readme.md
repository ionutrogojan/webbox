# Webbox

<p align="center"><img src="https://raw.githubusercontent.com/ionutrogojan/webbox/8f35d08f14740561eeb43aa785575d9928cff137/icon/webbox_icon.svg" width="480px"/></p>

### A light weight cli tool writen in Rust for its blazingly fast execution while maintaining its safety, blazingly fast 😎

---

### Available on some major platforms:

- [x] [Linux](https://github.com/ionutrogojan/webbox/releases/tag/0.1.0) - v0.2.0
- [x] [Windows](https://github.com/ionutrogojan/webbox/releases/tag/0.1.0) - v0.2.0
- [x] [MacOS](https://github.com/ionutrogojan/webbox/releases/tag/0.1.0) - v0.1.1
- [x] [Source](https://github.com/ionutrogojan/webbox/releases/tag/0.1.0) - v0.2.0

---

## Quick Setup

When you first open the program, there will be no `config.webbox` file. The program creates a new file in the "HOME" directory and exits.

Once the file is created, run the program again and it should open, blazingly fast 😎

After the initial execution, use `./webbox -help` to view the available arguments

⚠️ - do not manually edit the file and doing so can result in breaking the program. You have been warn.
If the program does happen to break, delete the `config.webbox` file and run the binary without arguments in order to create a new config file using the internal template.

---

## I can't find my `config.webbox` file! Where is it?

Running `./webbox -config-path` will give you the path to the file but that file.

⚠️ - running webbox with a system link such as a `.desktop` (linux) or `.link` (windows) file will not use the same config file as the one output by `./webbox -config-path` because system links run globaly on your system so the correct path to the `config.webbox` file is as follows:

```ini
[Linux / MacOS]

You will find the config.webbox file inside the "HOME" directory

[Windows]

You will find config.webbox inside the same directory as the .exe file

```

// TODO: force config.webbox file path to always use the user's home directory for consistance and ease of use

---

## Create a shortcut for blanzingly fast 😎 access

```ini
[Linux]

1. open webbox.desktop
2. change:
  Icon=/path_to_icon/webbox_icon.svg
  Exec=/path_to_executable/webbox
3. mv webbox.desktop ~/.local/share/applications
4. reboot system to apply changes

[Windows]

# there is no way to create a shotcut without the gui
1. right-click the desktop
2. new > shortcut
3. browse and select the .exe path
4. type the name "webbox" and finish
5. right-click the shortcut > properties > change icon
6. confirm the error > browse and select the .ico path

[MacOS]

# it's not pretty but it works
1. open finder and make a new folder inside of ~/applications
2. move the webbox file inside of it
3. drag-and-drop the file on the desktop to create a shortcut
# still working on adding an icon, to be determined on future versions

```

TODO:
  - [ ] better error messages
  - [ ] code refactor and cleanup
  - [ ] macOs icon [?](https://eshop.macsales.com/blog/28492-create-your-own-custom-icons-in-10-7-5-or-later/)
  - [ ] consistant config path [?](https://docs.rs/dirs/1.0.5/dirs/index.html)