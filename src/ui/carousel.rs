use iced::Background;
use iced::widget::{Stack, container, image, row};

use crate::domain::game::Game;
use crate::ui::{carousel_layout::CarouselLayout, theme};

pub fn create_carousel_row<'a, Message: 'static>(games: &[&Game], layout: &CarouselLayout, mode: crate::Mode) -> iced::widget::Row<'a, Message> {
    let mut carousel_row = row![].spacing(CarouselLayout::spacing()).align_y(iced::Alignment::Center);

    for (index, game) in games.iter().enumerate() {
        let carousel_item = create_carousel_item(game, layout, index, mode);
        carousel_row = carousel_row.push(carousel_item);
    }

    carousel_row
}

pub fn create_carousel_container<'a, Message: 'a>(carousel_games: iced::widget::Row<'a, Message>, layout: &CarouselLayout) -> iced::widget::Container<'a, Message> {
    container(carousel_games).padding(iced::Padding { top: 0.0, right: layout.canvas_padding(), bottom: 0.0, left: layout.canvas_padding() }).center_x(iced::Fill).style(|_theme| {
        container::Style {
            background: Some(iced::Background::Color(theme::BACKGROUND_COLOR)),
            border: iced::Border { color: theme::BORDER_COLOR, width: 0.0, radius: iced::border::Radius::from(0.0) },
            ..Default::default()
        }
    })
}

fn create_carousel_item<Message: 'static>(game: &Game, layout: &CarouselLayout, index: usize, mode: crate::Mode) -> iced::widget::Container<'static, Message> {
    let width = layout.game_width(index);
    let height = layout.game_height(index);
    let box_art_path = game.media_set().box_front_2d_thumbnail().path();

    let img = create_game_cover(box_art_path, width, height);
    let container = create_game_container(img, width, height);

    if mode == crate::Mode::Manage && game.is_hidden() {
        let overlay = container::Container::new(iced::widget::Space::new(iced::Length::Fixed(width), iced::Length::Fixed(height)))
            .width(iced::Length::Fixed(width))
            .height(iced::Length::Fixed(height))
            .style(|_theme| container::Style { background: Some(Background::Color(theme::HIDDEN_OVERLAY_COLOR)), ..Default::default() });

        container::Container::new(Stack::new().push(container).push(overlay)).width(iced::Length::Fixed(width)).height(iced::Length::Fixed(height))
    } else {
        container
    }
}

fn create_game_cover(box_art_path: &std::path::Path, width: f32, height: f32) -> iced::widget::Image {
    image(box_art_path.to_string_lossy().to_string()).width(iced::Length::Fixed(width)).height(iced::Length::Fixed(height)).content_fit(iced::ContentFit::Fill)
}

fn create_game_container<Message>(img: iced::widget::Image, width: f32, height: f32) -> iced::widget::Container<'static, Message> {
    container(img)
        .width(iced::Length::Fixed(width))
        .height(iced::Length::Fixed(height))
        .center_x(iced::Length::Fixed(width))
        .center_y(iced::Length::Fixed(height))
        .style(|_theme| container::Style { background: Some(iced::Background::Color(theme::BACKGROUND_COLOR)), ..Default::default() })
}
