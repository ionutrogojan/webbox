# Webbox

<p align="center"><img src="https://raw.githubusercontent.com/ionutrogojan/webbox/65bc4cd2983f8891e8b000b1d178fe7531beb698/webbox_icon.svg" width="128px"/></p>

### A light weight cli tool writen in Rust for its blazingly fast execution while maintaining its safety, blazingly fast 😎

---

### Available on some major platforms:

- [x] [Linux](https://github.com/ionutrogojan/webbox/releases/tag/0.1.0)
- [x] [Windows](https://github.com/ionutrogojan/webbox/releases/tag/0.1.0)
- [x] [MacOS](https://github.com/ionutrogojan/webbox/releases/tag/0.1.0)

---

## Quick Setup

When you first open the program, there will be no `config.webbox` file. The program creates a new file in the "HOME" directory and exits.

Once the file is created, run the program again and it should open, blazingly fast 😎

After the initial execution, use `./webbox -help` to view the available arguments

⚠️ - do not manually edit the file. Doing so can result in breaking the program. Do so at your own risk.
If the program does happen to break, delete the `config.webbox` file and run the binary without arguments in order to create a new config file using the internal template.

---

## I can't find my `config.webbox` file! Where is it?

```ini
[Linux]

You will find the config.webbox file inside the "HOME" directory

[Windows]

You will find config.webbox inside the same directory as the .exe file

[MacOs]

You will find the config.webbox file inside the "HOME" directory
```

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
5. right-click the shortcut > porperties > change icon
6. confirm the error > browse and select the .ico path

[MacOS]

# it's not pretty but it works
1. open finder and make a new folder inside of ~/applications
2. move the webbox file inside of it
3. drag-and-drop the file on the desktop to create a shortcut
# still working on adding an icon, to be determined on future versions

```
