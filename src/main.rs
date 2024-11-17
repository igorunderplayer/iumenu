use std::collections::HashMap;
use std::path::Path;
use std::{fs, process};

use gtk::gdk_pixbuf::Pixbuf;
use gtk::{
    prelude::*, Entry, Grid, Image, Label, ListBox, ListBoxRow, ScrolledWindow, SearchEntry,
};
use gtk::{Window, WindowType};
use ini::Ini;

pub struct App {
    pub id: u32,
    pub name: String,
    pub command: String,
}

impl App {
    pub fn new(id: u32, name: String, command: String) -> Self {
        Self { id, name, command }
    }
}

#[derive(Clone)]
struct DesktopApp {
    pub id: String,
    pub name: String,
    pub exec: String,
    pub keywords: String, // TODO: Implement this as list of string
    pub app_type: String,
    pub categories: String,
    pub icon: String,
}

impl DesktopApp {
    pub fn new(
        id: String,
        name: String,
        exec: String,
        keywords: String,
        app_type: String,
        categories: String,
        icon: String,
    ) -> Self {
        Self {
            id,
            name,
            exec,
            keywords,
            app_type,
            categories,
            icon,
        }
    }
}

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let sys_apps = get_desktop_apps();

    let window = Window::new(WindowType::Popup);

    window.set_title("Menu");
    window.set_default_size(800, 400);
    window.set_decorated(false);
    window.set_resizable(false);

    let search_entry = SearchEntry::new();
    search_entry.set_vexpand(false);
    search_entry.set_hexpand(true);
    search_entry.set_height_request(72);

    let main_grid = Grid::new();

    main_grid.set_expand(true);
    main_grid.set_orientation(gtk::Orientation::Vertical);

    main_grid.add(&search_entry);

    let scroll = ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
    scroll.set_max_content_height(328);
    scroll.set_expand(true);
    let list_box = ListBox::new();

    list_box.set_expand(true);

    let mut data_map: HashMap<String, DesktopApp> = HashMap::new();

    let rows: Vec<ListBoxRow> = sys_apps
        .iter()
        .map(|app| {
            let row = gtk::ListBoxRow::new();

            let row_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);

            row.set_height_request(48);

            let label = Label::new(Some(&app.name));

            let app_icon = display_app_icon(&app.icon.to_owned());

            unsafe {
                row.set_data("app-id", app.id.to_owned());
            }

            data_map.insert(app.id.clone(), app.clone());

            row_box.add(&app_icon);
            row_box.add(&label);
            row.add(&row_box);

            list_box.add(&row);

            row
        })
        .collect();

    list_box.connect_row_activated({
        let data_map = data_map.clone();
        move |list_box, row| {
            println!("escolheu algo");
            let mut id = String::from("");

            unsafe {
                id = row.data::<String>("app-id").unwrap().as_ref().to_owned();
            }

            let app_data = data_map.get(&id).unwrap();

            println!("name: {}", app_data.name);
            println!("exec: {}", app_data.exec);

            run_command(&app_data.exec);
        }
    });

    search_entry.connect_search_changed({
        let data_map = data_map.clone();
        let rows = rows.clone();
        move |entry| {
            let query = entry.text().to_lowercase();

            for row in &rows {
                if let Some(row_box) = row.child().and_then(|c| c.downcast::<gtk::Box>().ok()) {
                    for child in row_box.children() {
                        if let Some(label) = child.downcast_ref::<Label>() {
                            let label_text = label.text().to_string();
                            let mut keywords = String::from("");
                            unsafe {
                                let id = row.data::<String>("app-id").unwrap().as_ref().to_owned();
                                let app_data = data_map.get(&id).unwrap();
                                keywords = app_data.keywords.clone();
                            }
                            row.set_visible(
                                label_text.to_lowercase().contains(&query)
                                    || keywords.to_lowercase().contains(&query),
                            );
                        }
                    }
                }
            }
        }
    });

    scroll.add(&list_box);
    main_grid.add(&scroll);

    window.add(&main_grid);

    window.show_all();

    window.connect_hide(|window| {
        window.close();
        process::exit(0);
    });

    gtk::main();
}

fn run_command(command: &String) {
    let mut parts: Vec<&str> = command.split_whitespace().collect();

    if let Some(cmd) = parts.clone().get(0) {
        parts.remove(0);

        let _ = std::process::Command::new(cmd)
            .args(parts)
            .spawn()
            .expect("Command failed to start");
    }
}

fn get_desktop_apps() -> Vec<DesktopApp> {
    let mut apps = Vec::new();
    let dir = Path::new("/usr/share/applications");

    if dir.exists() && dir.is_dir() {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().map(|e| e == "desktop").unwrap_or(false) {
                        if let Some(app_name) = path.file_stem() {
                            let id = app_name.to_string_lossy().into_owned();
                            let app_data = parse_desktop_file(&path.to_str().unwrap(), id).unwrap();

                            if app_data.app_type != "Application" {
                                continue;
                            }

                            apps.push(app_data);
                        }
                    }
                }
            }
        }
    }
    apps
}

fn parse_desktop_file(path: &str, id: String) -> Option<DesktopApp> {
    let file_content = fs::read_to_string(&path).expect(&format!("Cant read file: {}", path));
    let ini = Ini::load_from_str(&file_content).expect("Cant read ini file");

    if let Some(section) = ini.section(Some("Desktop Entry")) {
        let name = section.get("Name").unwrap_or("Unknown").to_string();
        let icon = section.get("Icon").unwrap_or("").to_string();
        let app_type = section.get("Type").unwrap_or("").to_string();
        let categories = section.get("Categories").unwrap_or("Unknown").to_string();
        let keywords = section.get("Keywords").unwrap_or("").to_string();
        let exec_raw = section.get("Exec").unwrap_or("").to_string();

        let exec = clean_exec(&exec_raw);

        return Some(DesktopApp::new(
            id, name, exec, keywords, app_type, categories, icon,
        ));
    }

    None
}

fn clean_exec(exec: &str) -> String {
    let placeholders = ["%U", "%u", "%F", "%f", "%i", "%c", "%k"];
    let mut cleaned = exec.to_string();

    for placeholder in placeholders {
        cleaned = cleaned.replace(placeholder, "");
    }

    cleaned.trim().to_string()
}

fn load_icon(icon_name: &str) -> Option<Pixbuf> {
    let icon_theme = gtk::IconTheme::default().unwrap();

    if let Ok(Some(pixbuf)) = icon_theme.load_icon(icon_name, 32, gtk::IconLookupFlags::USE_BUILTIN)
    {
        Some(pixbuf)
    } else {
        None
    }
}

fn display_app_icon(icon_name: &str) -> gtk::Image {
    if let Some(pixbuf) = load_icon(icon_name) {
        let image = Image::from_pixbuf(Some(&pixbuf));
        image
    } else {
        Image::new()
    }
}
