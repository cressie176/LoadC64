use std::path::PathBuf;

use iced::keyboard::{Key, key};
use iced::widget::{column, container, image, row, text};
use iced::{Element, Task};

mod domain;
mod infrastructure;

use domain::cursor::Cursor;
use domain::library::Library;
use domain::section::CharacterSection;
use infrastructure::game_loader;

fn main() -> iced::Result {
    iced::application("Load!64", Load64::update, Load64::view)
        .subscription(Load64::subscription)
        .run_with(Load64::new)
}

struct Load64 {
    library: Library<CharacterSection>,
    cursor: Option<Cursor>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    NextGame,
    PreviousGame,
    NextSection,
    PreviousSection,
    ToSection(char),
}

impl Load64 {
    fn new() -> (Self, Task<Message>) {
        let mut library = Library::new(Box::new(CharacterSection::new));

        let games_dir = PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
            .join("Documents")
            .join("Load64")
            .join("games");

        match game_loader::load_games_from_directory(&games_dir) {
            Ok(games) => {
                for game in games {
                    library.add_game(game);
                }
            }
            Err(e) => {
                eprintln!("Failed to load games: {e}");
            }
        }

        let cursor = library.get_cursor();

        (Self { library, cursor }, Task::none())
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::NextGame => {
                if let Some(cursor) = &self.cursor {
                    self.cursor = self.library.next_game(cursor);
                }
            }
            Message::PreviousGame => {
                if let Some(cursor) = &self.cursor {
                    self.cursor = self.library.previous_game(cursor);
                }
            }
            Message::NextSection => {
                if let Some(cursor) = &self.cursor {
                    self.cursor = self.library.next_section(cursor);
                }
            }
            Message::PreviousSection => {
                if let Some(cursor) = &self.cursor {
                    self.cursor = self.library.previous_section(cursor);
                }
            }
            Message::ToSection(c) => {
                let section_key = c.to_string();
                if let Some(new_cursor) = self.library.to_section(&section_key) {
                    self.cursor = Some(new_cursor);
                }
            }
        }
    }

    #[allow(clippy::unused_self)]
    fn view(&self) -> Element<'_, Message> {
        let carousel = self.build_carousel();
        let game_info = self.build_game_info();

        let content = column![carousel, game_info].spacing(20);

        container(content).padding(20).center_x(iced::Fill).center_y(iced::Fill).into()
    }

    fn build_carousel(&self) -> Element<'_, Message> {
        if let Some(cursor) = &self.cursor {
            let games = self.library.get_game_window(cursor, -4, 9);

            if let Some(games) = games {
                let mut carousel_row = row![].spacing(10).align_y(iced::Alignment::Center);

                for (index, game) in games.iter().enumerate() {
                    let is_current = index == 4;
                    let (width, height) = if is_current { (234, 416) } else { (180, 320) };

                    let box_image = game.visit(|_title, _year, _publisher, _notes, media_set, _roms| media_set.box_front_2d().map(|media| media.path().clone()));

                    let box_content: Element<'_, Message> = box_image.map_or_else(
                        || {
                            container(text(""))
                                .width(width)
                                .height(height)
                                .style(|_theme| container::Style {
                                    background: Some(iced::Background::Color(iced::Color::from_rgb(0.0, 0.0, 0.0))),
                                    ..Default::default()
                                })
                                .into()
                        },
                        |path| {
                            let img = image(path.to_string_lossy().to_string()).content_fit(iced::ContentFit::Contain);

                            container(container(img).width(width).height(height).center_x(width).center_y(height))
                                .width(width)
                                .height(height)
                                .style(|_theme| container::Style {
                                    background: Some(iced::Background::Color(iced::Color::from_rgb(0.0, 0.0, 0.0))),
                                    ..Default::default()
                                })
                                .into()
                        },
                    );

                    carousel_row = carousel_row.push(box_content);
                }

                return container(carousel_row).center_x(iced::Fill).into();
            }
        }

        text("No games available").into()
    }

    fn build_game_info(&self) -> Element<'_, Message> {
        if let Some(cursor) = &self.cursor {
            let games = self.library.get_game_window(cursor, -4, 9);

            if let Some(games) = games
                && let Some(current_game) = games.get(4)
            {
                let (title, metadata) = current_game.visit(|title, year, publisher, _notes, _media_set, _roms| {
                    let mut metadata_parts = Vec::new();
                    if let Some(y) = year {
                        metadata_parts.push(y.to_string());
                    }
                    if let Some(p) = publisher {
                        metadata_parts.push(p.to_string());
                    }

                    let metadata_text = if metadata_parts.is_empty() { None } else { Some(metadata_parts.join(" - ")) };

                    (title.to_string(), metadata_text)
                });

                let mut info = column![text(title).size(30)].spacing(5).align_x(iced::alignment::Horizontal::Center);

                if let Some(metadata_text) = metadata {
                    info = info.push(text(metadata_text).size(18));
                }

                return container(info).center_x(iced::Fill).into();
            }
        }

        text("").into()
    }

    #[allow(clippy::unused_self)]
    fn subscription(&self) -> iced::Subscription<Message> {
        iced::keyboard::on_key_press(|key, _modifiers| match key {
            Key::Named(key::Named::ArrowLeft) => Some(Message::PreviousGame),
            Key::Named(key::Named::ArrowRight) => Some(Message::NextGame),
            Key::Named(key::Named::PageUp) => Some(Message::PreviousSection),
            Key::Named(key::Named::PageDown) => Some(Message::NextSection),
            Key::Character(c) => {
                if let Some(first_char) = c.chars().next()
                    && first_char.is_alphanumeric()
                {
                    return Some(Message::ToSection(first_char));
                }
                None
            }
            _ => None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_initializes() {
        let (_app, _task) = Load64::new();
    }

    #[test]
    fn test_greeting_message() {
        let greeting = "Hello Load!64";
        assert_eq!(greeting, "Hello Load!64");
    }
}
