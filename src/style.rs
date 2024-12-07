use gdk::{pango::Style, Display};
use gtk::{
    prelude::{CssProviderExt, StyleContextExt},
    CssProvider, StyleContext,
};

pub fn apply_custom_css(path: &str) {
    let provider = CssProvider::new();

    if !provider.load_from_path(path).is_err() {
        if let Some(screen) = gdk::Screen::default() {
            StyleContext::add_provider_for_screen(
                &screen,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    } else {
        eprintln!("Could not load css file: {}", path);
    }
}
