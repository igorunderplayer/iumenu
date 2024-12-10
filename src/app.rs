use std::collections::HashMap;

use iced::{
    event,
    keyboard::{key::Named, Key},
    widget::{button, column, scrollable, text, text_input, Column},
    Event, Length, Subscription, Task, Theme,
};

use crate::{
    action::click_app,
    freedesktop::desktop_entry::{get_available_apps, DesktopApp},
};

struct Entry {
    visible: bool,
    id: String,
}

#[derive(Default)]
struct IUMenu {
    search_content: String,
    entries: Vec<Entry>,
    apps: HashMap<String, DesktopApp>,
    selected_index: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    SearchChanged(String),
    ButtonClicked(String),
    EventOccurred(Event),
    OnSubmit,
}

impl IUMenu {
    pub fn new() -> (Self, Task<Message>) {
        let apps = get_available_apps();

        let mut entries: Vec<Entry> = apps
            .iter()
            .map(|(id, _)| Entry {
                visible: true,
                id: id.clone(),
            })
            .collect();

        entries.sort_by(|a, b| {
            let app_a = &apps[&a.id];
            let app_b = &apps[&b.id];
            app_a.name.to_lowercase().cmp(&app_b.name.to_lowercase())
        });

        (
            Self {
                search_content: String::default(),
                entries,
                apps,
                selected_index: 0,
            },
            Task::none(),
        )
    }

    pub fn view(&self) -> Column<Message> {
        let column = self
            .entries
            .iter()
            .filter(|entry| entry.visible)
            .enumerate()
            .fold(Column::new().padding(8), |col, (index, entry)| {
                let app = self.apps.get(&entry.id).unwrap();
                col.push(
                    button(column![
                        text(app.name.clone()),
                        text(app.comment.clone()).style(|theme| text::secondary(theme))
                    ])
                    .on_press(Message::ButtonClicked(entry.id.to_owned()))
                    .width(Length::Fill)
                    .padding(8)
                    .style(move |theme, status| {
                        if self.selected_index == index {
                            let mut style = button::primary(theme, status).clone();
                            style.border = iced::Border::default().rounded(8);
                            style
                        } else {
                            button::text(theme, status)
                        }
                    }),
                )
            });

        let scroll = scrollable(column).width(Length::Fill);

        let input = text_input("Pesquisa ai duvido", &self.search_content)
            .padding(16)
            .style(|theme, status| {
                let mut style = text_input::default(theme, status);
                style.border = iced::Border::default()
                    .rounded(8)
                    .color(theme.palette().primary)
                    .width(1);
                style
            })
            .on_input(Message::SearchChanged)
            .on_submit(Message::OnSubmit);

        column![input, scroll].padding(8)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SearchChanged(value) => {
                self.search_content = value;
                self.selected_index = 0;

                for entry in &mut self.entries {
                    let app = self.apps.get(&entry.id).unwrap();
                    entry.visible = app
                        .name
                        .to_lowercase()
                        .contains(&self.search_content.to_lowercase())
                        || app
                            .keywords
                            .to_lowercase()
                            .contains(&self.search_content.to_lowercase());
                }

                return Task::none();
            }
            Message::ButtonClicked(app_id) => {
                println!("botao clicado, {}", app_id);
                if let Some(app) = self.apps.get(&app_id) {
                    click_app(app);
                }
                return Task::none();
            }
            Message::OnSubmit => self.on_execute(),
            Message::EventOccurred(event) => {
                if let Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    key,
                    modified_key: _,
                    physical_key: _,
                    location: _,
                    modifiers: _,
                    text: _,
                }) = event
                {
                    return self.handle_input(key);
                }

                return Task::none();
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::EventOccurred)
    }

    fn theme(&self) -> Theme {
        iced::Theme::CatppuccinMocha.clone()
    }

    fn on_execute(&self) -> Task<Message> {
        let filtered: Vec<&Entry> = self.entries.iter().filter(|p| p.visible).collect();
        let entry = filtered[self.selected_index];

        if let Some(app) = self.apps.get(&entry.id) {
            click_app(app);
        }

        Task::none()
    }

    fn handle_input(&mut self, key_code: Key) -> Task<Message> {
        match key_code {
            Key::Named(Named::ArrowUp) => {
                return self.dec_selected();
            }
            Key::Named(Named::ArrowDown) => {
                return self.inc_selected();
            }
            Key::Named(Named::Enter) => return self.on_execute(),
            Key::Named(Named::Escape) => {
                std::process::exit(0);
            }
            _ => {}
        };

        Task::none()
    }

    fn inc_selected(&mut self) -> Task<Message> {
        if self.selected_index + 1 < self.entries.iter().filter(|e| e.visible).count() {
            self.selected_index += 1;
        }
        Task::none()
    }

    fn dec_selected(&mut self) -> Task<Message> {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
        Task::none()
    }
}

pub fn run() -> iced::Result {
    let app = iced::application("IUMenu", IUMenu::update, IUMenu::view);

    app.window(iced::window::Settings {
        size: iced::Size {
            width: 800.0,
            height: 400.0,
        },
        decorations: false,
        resizable: false,
        position: iced::window::Position::Centered,
        level: iced::window::Level::AlwaysOnTop,
        transparent: true,
        min_size: None,
        max_size: None,
        ..Default::default()
    })
    .settings(iced::Settings {
        id: Some("com.igorunderplayer.iumenu".to_owned()),
        ..Default::default()
    })
    .theme(IUMenu::theme)
    .subscription(IUMenu::subscription)
    .run_with(IUMenu::new)
}
