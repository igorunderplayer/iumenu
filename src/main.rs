use freedesktop::desktop_entry::DesktopApp;

mod action;
mod app;
mod args;
mod config;
mod freedesktop;
mod util;

fn main() -> iced::Result {
    app::run()
}
