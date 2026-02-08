use iced::Element;
use iced::widget::{column, container, text};

use crate::domain::game::Game;

pub fn create_game_info<'a, Message: 'a>(game: &'a Game) -> Element<'a, Message> {
    let title = game.title().to_string();
    let mut metadata_parts = Vec::new();
    if let Some(year) = game.year() {
        metadata_parts.push(year.to_string());
    }
    if let Some(publisher) = game.publisher() {
        metadata_parts.push(publisher.to_string());
    }

    let metadata = if metadata_parts.is_empty() { None } else { Some(metadata_parts.join(" - ")) };

    let info: iced::widget::Column<'_, Message> = if let Some(metadata_text) = metadata {
        column![text(title).size(30).color(iced::Color::WHITE), text(metadata_text).size(18).color(iced::Color::WHITE)].spacing(5).align_x(iced::alignment::Horizontal::Center)
    } else {
        column![text(title).size(30).color(iced::Color::WHITE)].spacing(5).align_x(iced::alignment::Horizontal::Center)
    };

    container(info).center_x(iced::Fill).into()
}
