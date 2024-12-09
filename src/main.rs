use freedesktop::desktop_entry::DesktopApp;

mod app;
mod args;
mod config;
mod freedesktop;
mod util;

fn main() -> iced::Result {
    app::run()
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

pub fn click_app(app: &DesktopApp) {
    println!("name: {}", app.name);
    println!("exec: {}", app.exec);

    run_command(&app.exec);
}
