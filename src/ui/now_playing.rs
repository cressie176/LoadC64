use crate::ui::theme;
use iced::widget::{button, column, container, text};
use iced::{Element, Length};

pub fn view<Message: Clone + 'static>(game_name: &str, quit_message: Message) -> Element<'_, Message> {
    let now_playing_text = text("Now Playing").size(theme::TITLE_FONT_SIZE).color(theme::TEXT_COLOR);

    let game_name_text = text(game_name).size(theme::TITLE_FONT_SIZE * 1.5).color(theme::TEXT_COLOR);

    let quit_button = button(text("Quit Game").size(theme::METADATA_FONT_SIZE)).on_press(quit_message).padding(15);

    let content = column![now_playing_text, game_name_text, quit_button].spacing(30).align_x(iced::Alignment::Center);

    container(content).center_x(Length::Fill).center_y(Length::Fill).into()
}
