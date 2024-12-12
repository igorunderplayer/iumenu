use gtk::CssProvider;

pub fn apply_custom_css(path: &str) {
    let provider = CssProvider::new();

    provider.load_from_path(path);
    if let Some(display) = gtk::gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
