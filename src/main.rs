use std::collections::HashMap;
use std::env::args;
use std::path::Path;
use std::{fs, process};

use config::get_default_path;
use gdk::glib::Propagation;
use gdk::keys;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::{prelude::*, Grid, Image, Label, ListBox, ListBoxRow, ScrolledWindow, SearchEntry};
use gtk::{Window, WindowType};
use ini::Ini;

mod args;
mod config;

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
    let args = args::parse_arguments();
    let config = config::load_from_file(&args.config.unwrap_or(get_default_path()));
    gtk::init().expect("Failed to initialize GTK.");

    let sys_apps = get_desktop_apps();

    let window = Window::new(WindowType::Toplevel);

    window.set_title("Menu");
    window.set_default_size(config.window.width, config.window.height);
    window.set_decorated(false);
    window.set_resizable(false);
    window.set_position(gtk::WindowPosition::CenterAlways);
    window.set_window_position(gtk::WindowPosition::CenterAlways);
    window.set_keep_above(true);

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

    list_box.select_row(Some(&rows[0]));

    list_box.connect_row_activated({
        let data_map = data_map.clone();
        move |_list_box, row| {
            println!("escolheu algo");
            let mut id = String::from("");

            unsafe {
                id = row.data::<String>("app-id").unwrap().as_ref().to_owned();
            }

            let app_data = data_map.get(&id).unwrap();

            click_app(app_data);
        }
    });

    search_entry.connect_search_changed({
        let data_map = data_map.clone();
        let rows = rows.clone();
        let list_box = list_box.clone();
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
                                let keywords = app_data.keywords.clone();
                            }
                            row.set_visible(
                                label_text.to_lowercase().contains(&query)
                                    || keywords.to_lowercase().contains(&query),
                            );
                        }
                    }
                }
            }
            list_box.select_row(rows.iter().find(|p| p.is_visible()));
        }
    });

    search_entry.connect_key_press_event({
        let list_clone = list_box.clone();
        move |_entry, event| {
            let key = event.keyval();
            let visible_rows: Vec<ListBoxRow> = list_clone
                .children()
                .into_iter()
                .filter_map(|widget| widget.downcast::<ListBoxRow>().ok())
                .filter(|p| p.is_visible())
                .collect();

            let current_index = visible_rows
                .iter()
                .position(|row| list_clone.selected_row() == Some(row.clone()));

            match key {
                keys::constants::Return => {
                    if let Some(_index) = current_index {
                        if let Some(row) = list_clone.selected_row() {
                            println!("escolheu algo");
                            let id;

                            unsafe {
                                id = row.data::<String>("app-id").unwrap().as_ref().to_owned();
                            }

                            let app_data = data_map.get(&id).unwrap();
                            click_app(app_data);
                        }
                    }
                    Propagation::Stop
                }
                gdk::keys::constants::Down => {
                    if let Some(index) = current_index {
                        if index + 1 < visible_rows.len() {
                            list_clone.select_row(Some(&visible_rows[index + 1]));
                        }
                    } else if !visible_rows.is_empty() {
                        list_clone.select_row(Some(&visible_rows[0]));
                    }
                    Propagation::Stop
                }
                gdk::keys::constants::Up => {
                    if let Some(index) = current_index {
                        if index > 0 {
                            list_clone.select_row(Some(&visible_rows[index - 1]));
                        }
                    } else if !visible_rows.is_empty() {
                        list_clone.select_row(Some(&visible_rows[0]));
                    }
                    Propagation::Stop
                }
                _ => Propagation::Proceed,
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

fn click_app(app: &DesktopApp) {
    println!("name: {}", app.name);
    println!("exec: {}", app.exec);

    run_command(&app.exec);
}

fn get_desktop_apps() -> Vec<DesktopApp> {
    let mut apps = Vec::new();
    let dir = Path::new("/usr/share/applications");

    if dir.exists() && dir.is_dir() {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "desktop").unwrap_or(false) {
                    if let Some(app_name) = path.file_stem() {
                        let id = app_name.to_string_lossy().into_owned();
                        let app_data = parse_desktop_file(path.to_str().unwrap(), id).unwrap();

                        if app_data.app_type != "Application" {
                            continue;
                        }

                        apps.push(app_data);
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
        Image::from_pixbuf(Some(&pixbuf))
    } else {
        Image::new()
    }
}
