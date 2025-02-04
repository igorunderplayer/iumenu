use crate::AppEntry;

pub fn run_command(command: &String) {
    #[cfg(target_os = "windows")]
    {
        run_windows_command(command);
    }

    #[cfg(not(target_os = "windows"))]
    {
        let mut parts: Vec<&str> = command.split_whitespace().collect();

        if let Some(cmd) = parts.clone().get(0) {
            parts.remove(0);

            println!("cmd: {}", cmd);

            let _ = std::process::Command::new(cmd)
                .args(parts)
                .spawn()
                .expect("Command failed to start");
        }
    }
}

#[cfg(target_os = "windows")]
fn run_windows_command(command: &String) {
    let _ = std::process::Command::new(command)
        .spawn()
        .expect("Command failed to start");
}

pub fn click_app(app: &AppEntry) {
    println!("name: {}", app.name);
    println!("exec: {}", app.exec);

    run_command(&app.exec);
}
