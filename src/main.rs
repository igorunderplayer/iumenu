use std::collections::HashMap;
use std::path::Path;
use std::{fs, process};

use gtk::{prelude::*, Button, Entry, Grid, Label, ListBox, ScrolledWindow};
use gtk::{Window, WindowType};

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

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let apps = [
        App::new(1, String::from("Zed"), String::from("zeditor")),
        App::new(2, String::from("Visual Studio Code"), String::from("code")),
        App::new(3, String::from("Chrome"), String::from("chrome")),
        App::new(4, String::from("Firefox"), String::from("firefox")),
    ];

    let sys_apps = get_desktop_apps();

    let window = Window::new(WindowType::Popup);

    window.set_title("Menu");
    window.set_default_size(800, 400);
    window.set_decorated(false);
    window.set_resizable(false);

    let input = Entry::new();
    input.set_vexpand(false);
    input.set_hexpand(true);
    input.set_height_request(72);

    let main_grid = Grid::new();

    main_grid.set_expand(true);
    main_grid.set_orientation(gtk::Orientation::Vertical);

    main_grid.add(&input);

    let scroll = ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
    scroll.set_max_content_height(328);
    scroll.set_expand(true);
    let list = ListBox::new();

    list.set_expand(true);

    let mut data_map: HashMap<String, App> = HashMap::new();

    // for app in apps {
    //     let row = gtk::ListBoxRow::new();
    //     let text = Label::new(Some(&app.name));

    //     data_map.insert(app.command.clone(), app);

    //     row.add(&text);
    //     list.add(&row);
    // }

    // list.connect_row_activated(move |list_box, row| {
    //     println!("escolheu algo");
    //     if let Some(label) = row.child().and_then(|c| c.downcast::<Label>().ok()) {
    //         let text = label.text().to_string();

    //         println!("Você escolheu: {}", text);
    //         let (exec, app) = data_map
    //             .iter()
    //             .find(|p| p.1.name == text)
    //             .expect("App not found");

    //         run_command(exec);
    //     }
    // });

    for app in sys_apps {
        let row = gtk::ListBoxRow::new();
        let label = Label::new(Some(&app));

        row.add(&label);
        list.add(&row)
    }

    scroll.add(&list);
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
    std::process::Command::new(command)
        .spawn()
        .expect("Command failed to start");
}

fn get_desktop_apps() -> Vec<String> {
    let mut apps = Vec::new();
    let dir = Path::new("/usr/share/applications");

    if dir.exists() && dir.is_dir() {
        // Lê todos os arquivos no diretório
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().map(|e| e == "desktop").unwrap_or(false) {
                        if let Some(app_name) = path.file_stem() {
                            apps.push(app_name.to_string_lossy().into_owned());
                        }
                    }
                }
            }
        }
    }
    apps
}
