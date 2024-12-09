use std::collections::HashMap;

use iced::{
    event,
    keyboard::{key::Named, Key},
    widget::{button, column, scrollable, text, text_input, Button, Column, Scrollable, Text},
    Application, Element, Event, Length, Subscription, Task, Theme,
};

use crate::{click_app, get_desktop_apps, DesktopApp};

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
    selected_id: String,
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
        let apps = get_desktop_apps();
        let entries: Vec<Entry> = apps
            .keys()
            .cloned()
            .map(|k| Entry {
                visible: true,
                id: k,
            })
            .collect();
        (
            Self {
                search_content: String::default(),
                entries,
                apps,
                selected_index: 0,
                selected_id: String::default(),
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
            .fold(Column::new(), |col, (index, entry)| {
                let app = self.apps.get(&entry.id).unwrap();
                col.push(
                    Button::new(Text::new(app.name.clone()))
                        .on_press(Message::ButtonClicked(entry.id.to_owned()))
                        .width(Length::Fill)
                        .style(move |theme, status| {
                            if self.selected_index == index {
                                button::primary(theme, status)
                            } else {
                                button::text(theme, status)
                            }
                        }),
                )
            });

        let scroll = scrollable(column).width(Length::Fill);

        return column![
            text_input("Pesquisa ai duvido", &self.search_content)
                .on_input(Message::SearchChanged)
                .on_submit(Message::OnSubmit),
            scroll
        ];
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
                println!("Selected line : {:?}", self.selected_index);
                return self.dec_selected();
            }
            Key::Named(Named::ArrowDown) => {
                println!("Selected line : {:?}", self.selected_index);
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
        self.selected_index += 1;
        Task::none()
    }

    fn dec_selected(&mut self) -> Task<Message> {
        self.selected_index -= 1;
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
        ..Default::default()
    })
    .theme(IUMenu::theme)
    .subscription(IUMenu::subscription)
    .run_with(IUMenu::new)
}
