# IUMENU

A simple to use, (multiplatform?), GTK based and writen in Rust app launcher and menu!


## Building

> [!IMPORTANT]
> The project is incomplete, including this building section.


### Requirements:

#### Linux

[Reference](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_linux.html)

Fedora and derivatives:
`sudo dnf install gtk4-devel gcc`

Debian and derivatives:
`sudo apt install libgtk-4-dev build-essential`

Arch and derivatives:
`sudo pacman -S gtk4 base-devel`


#### macOS

Install gtk4 via homebrew

`brew install gtk4`


#### Windows

You will need to follow more steps.

I recomment you to follow gtk4-rs guide:
https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_windows.html


### Build and install using Cargo:
```sh
git clone https://github.com/igorunderplayer/iumenu.git
cd iumenu
cargo install --path .
```
