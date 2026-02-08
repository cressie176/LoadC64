use iced::widget::{column, container, image, row, text};
use iced::{Element, Task};

mod cli;
mod domain;
mod infrastructure;
mod ui;

use domain::cursor::Cursor;
use domain::library::Library;
use domain::section::CharacterSection;
use infrastructure::{game_loader, vice_emulator::ViceEmulator};
use ui::{carousel_layout::CarouselLayout, input};

const DEFAULT_WINDOW_WIDTH: f32 = 1280.0;

fn main() -> iced::Result {
    iced::application("Load!64", App::update, App::view).subscription(App::subscription).run_with(App::new)
}

struct App {
    library: Library<CharacterSection>,
    cursor: Option<Cursor>,
    window_width: f32,
    vice_emulator: ViceEmulator,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    WindowResized(f32, f32),
    NextGame,
    PreviousGame,
    NextSection,
    PreviousSection,
    ToSection(char),
    LaunchGame,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let args = cli::parse();
        let mut library = Library::new(Box::new(CharacterSection::new));
        game_loader::load_games_into(&mut library, &args.games_dir).expect("Error loading games");
        let cursor = library.get_cursor();
        let vice_emulator = ViceEmulator::new(args.vice_path);
        (Self { library, cursor, window_width: DEFAULT_WINDOW_WIDTH, vice_emulator }, Task::none())
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::WindowResized(width, _height) => {
                self.window_width = width;
            }
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
            Message::LaunchGame => {
                self.launch_current_game();
            }
        }
    }

    fn launch_current_game(&self) {
        let Some(cursor) = &self.cursor else {
            return;
        };

        let game = self.library.get_game(cursor);

        let Some(rom) = game.roms().first() else {
            return;
        };

        self.vice_emulator.launch(rom.path()).expect("Failed to launch VICE");
    }

    fn view(&self) -> Element<'_, Message> {
        let layout = CarouselLayout::new(self.window_width);
        let (carousel_games, game_info) = self.get_carousel_games(&layout);
        let carousel = Self::get_carousel_container(carousel_games, &layout);

        let content = column![carousel, game_info].spacing(20);

        container(content)
            .center_x(iced::Fill)
            .center_y(iced::Fill)
            .style(|_theme| container::Style { background: Some(iced::Background::Color(iced::Color::BLACK)), ..Default::default() })
            .into()
    }

    fn get_carousel_container<'a>(carousel_games: iced::widget::Row<'a, Message>, layout: &CarouselLayout) -> iced::widget::Container<'a, Message> {
        container(carousel_games).padding(iced::Padding { top: 0.0, right: layout.canvas_padding(), bottom: 0.0, left: layout.canvas_padding() }).center_x(iced::Fill).style(
            |_theme| container::Style {
                background: Some(iced::Background::Color(iced::Color::BLACK)),
                border: iced::Border { color: iced::Color::BLACK, width: 0.0, radius: iced::border::Radius::from(0.0) },
                ..Default::default()
            },
        )
    }

    fn get_carousel_games(&self, layout: &CarouselLayout) -> (iced::widget::Row<'_, Message>, Element<'_, Message>) {
        let mut carousel_games = row![].spacing(CarouselLayout::spacing()).align_y(iced::Alignment::Center);
        let mut game_info: Element<'_, Message> = container(text("")).into();

        if let Some(cursor) = &self.cursor {
            let games = self.library.get_game_window(cursor, layout.offset(), layout.number_of_games());

            if let Some(games) = games {
                for (index, game) in games.iter().enumerate() {
                    let carousel_item = Self::create_carousel_item(game, layout, index);
                    carousel_games = carousel_games.push(carousel_item);
                }

                if let Some(current_game) = games.get(layout.current_game_index()) {
                    game_info = Self::create_game_info(current_game);
                }
            }
        }

        (carousel_games, game_info)
    }

    fn create_game_info(game: &domain::game::Game) -> Element<'_, Message> {
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

    fn create_carousel_item(game: &domain::game::Game, layout: &CarouselLayout, index: usize) -> iced::widget::Container<'static, Message> {
        let width = layout.game_width(index);
        let height = layout.game_height(index);
        let box_art_path = game.media_set().box_front_2d_thumbnail().path();

        let img = Self::create_game_cover(box_art_path, width, height);
        Self::create_game_container(img, width, height)
    }

    fn create_game_cover(box_art_path: &std::path::Path, width: f32, height: f32) -> iced::widget::Image {
        image(box_art_path.to_string_lossy().to_string()).width(iced::Length::Fixed(width)).height(iced::Length::Fixed(height)).content_fit(iced::ContentFit::Fill)
    }

    fn create_game_container(img: iced::widget::Image, width: f32, height: f32) -> iced::widget::Container<'static, Message> {
        container(img)
            .width(iced::Length::Fixed(width))
            .height(iced::Length::Fixed(height))
            .center_x(iced::Length::Fixed(width))
            .center_y(iced::Length::Fixed(height))
            .style(|_theme| container::Style { background: Some(iced::Background::Color(iced::Color::BLACK)), ..Default::default() })
    }

    #[allow(clippy::unused_self)]
    fn subscription(&self) -> iced::Subscription<Message> {
        let window_events = iced::event::listen_with(|event, _status, _id| match event {
            iced::Event::Window(iced::window::Event::Resized(size)) => Some(Message::WindowResized(size.width, size.height)),
            _ => None,
        });

        let keyboard_events = iced::keyboard::on_key_press(|key, _modifiers| match key {
            iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowLeft) => Some(Message::PreviousGame),
            iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowRight) => Some(Message::NextGame),
            iced::keyboard::Key::Named(iced::keyboard::key::Named::PageUp) => Some(Message::PreviousSection),
            iced::keyboard::Key::Named(iced::keyboard::key::Named::PageDown) => Some(Message::NextSection),
            iced::keyboard::Key::Named(iced::keyboard::key::Named::Enter) => Some(Message::LaunchGame),
            iced::keyboard::Key::Character(c) => {
                if let Some(first_char) = c.chars().next()
                    && first_char.is_alphanumeric()
                {
                    return Some(Message::ToSection(first_char));
                }
                None
            }
            _ => None,
        });

        let gamepad_events =
            iced::Subscription::run(|| input::gamepad_worker(Message::PreviousGame, Message::NextGame, Message::PreviousSection, Message::NextSection, Message::LaunchGame));

        iced::Subscription::batch(vec![window_events, keyboard_events, gamepad_events])
    }
}
