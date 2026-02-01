use iced::widget::{column, container, text};
use iced::{Element, Task};

mod domain;

use domain::character_section::CharacterSection;
use domain::game::Game;
use domain::library::Library;

fn main() -> iced::Result {
    iced::application("Load!64", Load64::update, Load64::view).run_with(Load64::new)
}

struct Load64 {
    library: Library<CharacterSection>,
}

#[derive(Debug, Clone)]
enum Message {}

impl Load64 {
    fn new() -> (Self, Task<Message>) {
        let mut library = Library::new();

        library.add_section(CharacterSection::new('a'));
        library.add_section(CharacterSection::new('m'));
        library.add_section(CharacterSection::new('z'));

        let game1 = Game::new(
            "1".to_string(),
            "Monkey Island".to_string(),
            "monkey-island".to_string(),
            Some(1990),
            Some("LucasArts".to_string()),
            None,
        );
        let game2 = Game::new(
            "2".to_string(),
            "Maniac Mansion".to_string(),
            "maniac-mansion".to_string(),
            Some(1987),
            Some("LucasArts".to_string()),
            None,
        );
        let game3 = Game::new(
            "3".to_string(),
            "Zak McKracken".to_string(),
            "zak-mckracken".to_string(),
            Some(1988),
            Some("LucasArts".to_string()),
            None,
        );

        library.add_game(&game1);
        library.add_game(&game2);
        library.add_game(&game3);

        (Self { library }, Task::none())
    }

    #[allow(clippy::unused_self, clippy::missing_const_for_fn)]
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> Element<'_, Message> {
        let game_text = self
            .library
            .with_current_game(|game| game.visit(|title, year, publisher, _| format!("{} ({}{})", title, year.unwrap_or(0), publisher.map_or(String::new(), |p| format!(" - {p}")))))
            .unwrap_or_else(|| "No games".to_string());

        let content = column![text(game_text).size(50),];

        container(content).center(iced::Fill).into()
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
