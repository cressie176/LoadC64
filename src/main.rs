use std::time::Duration;

use gilrs::{Axis, Button, Event, EventType, Gilrs};
use iced::keyboard::{Key, key};
use iced::widget::{column, container, image, row, text};
use iced::{Element, Task};

mod cli;
mod domain;
mod infrastructure;
mod ui;

use domain::cursor::Cursor;
use domain::library::Library;
use domain::rom::Rom;
use domain::section::CharacterSection;
use infrastructure::{game_loader, vice_emulator::ViceEmulator};
use ui::carousel_layout::CarouselLayout;

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

    fn launch_current_game(&self) {
        let Some(cursor) = &self.cursor else {
            return;
        };

        let game = self.library.get_game(cursor);

        game.visit(|_title, _year, _publisher, _notes, _media_set, roms: &[Rom]| {
            let Some(rom) = roms.first() else {
                return;
            };

            self.vice_emulator.launch(rom.path()).expect("Failed to launch VICE");
        });
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

    #[allow(clippy::too_many_lines)]
    fn view(&self) -> Element<'_, Message> {
        let layout = CarouselLayout::new(self.window_width);

        let mut carousel_row = row![].spacing(CarouselLayout::spacing()).align_y(iced::Alignment::Center);

        if let Some(cursor) = &self.cursor {
            let games = self.library.get_game_window(cursor, layout.offset(), layout.number_of_games());

            if let Some(games) = games {
                for (index, game) in games.iter().enumerate() {
                    let width = layout.game_width(index);
                    let height = layout.game_height(index);
                    let box_image = game.visit(|_title, _year, _publisher, _notes, media_set, _roms| media_set.box_front_2d_thumbnail().map(|media| media.path().clone()));

                    let game_container = box_image.map_or_else(
                        || {
                            container(text(""))
                                .width(iced::Length::Fixed(width))
                                .height(iced::Length::Fixed(height))
                                .style(|_theme| container::Style { background: Some(iced::Background::Color(iced::Color::BLACK)), ..Default::default() })
                        },
                        |path| {
                            let img =
                                image(path.to_string_lossy().to_string()).width(iced::Length::Fixed(width)).height(iced::Length::Fixed(height)).content_fit(iced::ContentFit::Fill);

                            container(img)
                                .width(iced::Length::Fixed(width))
                                .height(iced::Length::Fixed(height))
                                .center_x(iced::Length::Fixed(width))
                                .center_y(iced::Length::Fixed(height))
                                .style(|_theme| container::Style { background: Some(iced::Background::Color(iced::Color::BLACK)), ..Default::default() })
                        },
                    );

                    carousel_row = carousel_row.push(game_container);
                }
            }
        }

        let carousel = container(carousel_row)
            .padding(iced::Padding { top: 0.0, right: layout.canvas_padding(), bottom: 0.0, left: layout.canvas_padding() })
            .center_x(iced::Fill)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(iced::Color::BLACK)),
                border: iced::Border { color: iced::Color::BLACK, width: 0.0, radius: iced::border::Radius::from(0.0) },
                ..Default::default()
            });

        #[allow(clippy::option_if_let_else)]
        let game_info: Element<'_, Message> = if let Some(cursor) = &self.cursor {
            let games = self.library.get_game_window(cursor, layout.offset(), layout.number_of_games());

            #[allow(clippy::option_if_let_else)]
            if let Some(games) = games {
                #[allow(clippy::option_if_let_else)]
                if let Some(current_game) = games.get(layout.current_game_index()) {
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

                    let info: iced::widget::Column<'_, Message> = if let Some(metadata_text) = metadata {
                        column![text(title).size(30).color(iced::Color::WHITE), text(metadata_text).size(18).color(iced::Color::WHITE)]
                            .spacing(5)
                            .align_x(iced::alignment::Horizontal::Center)
                    } else {
                        column![text(title).size(30).color(iced::Color::WHITE)].spacing(5).align_x(iced::alignment::Horizontal::Center)
                    };

                    container(info).center_x(iced::Fill).into()
                } else {
                    container(text("")).into()
                }
            } else {
                container(text("")).into()
            }
        } else {
            container(text("")).into()
        };

        let content = column![carousel, game_info].spacing(20);

        container(content)
            .center_x(iced::Fill)
            .center_y(iced::Fill)
            .style(|_theme| container::Style { background: Some(iced::Background::Color(iced::Color::BLACK)), ..Default::default() })
            .into()
    }

    #[allow(clippy::unused_self)]
    fn subscription(&self) -> iced::Subscription<Message> {
        let window_events = iced::event::listen_with(|event, _status, _id| match event {
            iced::Event::Window(iced::window::Event::Resized(size)) => Some(Message::WindowResized(size.width, size.height)),
            _ => None,
        });

        let keyboard_events = iced::keyboard::on_key_press(|key, _modifiers| match key {
            Key::Named(key::Named::ArrowLeft) => Some(Message::PreviousGame),
            Key::Named(key::Named::ArrowRight) => Some(Message::NextGame),
            Key::Named(key::Named::PageUp) => Some(Message::PreviousSection),
            Key::Named(key::Named::PageDown) => Some(Message::NextSection),
            Key::Named(key::Named::Enter) => Some(Message::LaunchGame),
            Key::Character(c) => {
                if let Some(first_char) = c.chars().next()
                    && first_char.is_alphanumeric()
                {
                    return Some(Message::ToSection(first_char));
                }
                None
            }
            _ => None,
        });

        let gamepad_subscription = iced::Subscription::run(gamepad_worker);

        iced::Subscription::batch(vec![window_events, keyboard_events, gamepad_subscription])
    }
}

fn gamepad_worker() -> impl iced::futures::Stream<Item = Message> {
    use iced::futures::stream::StreamExt;

    iced::stream::channel(50, move |mut output| async move {
        let mut gilrs = match Gilrs::new() {
            Ok(g) => g,
            Err(_) => return,
        };

        let mut interval = async_std::stream::interval(Duration::from_millis(16));
        let mut left_stick_x = 0.0_f32;
        let mut frame_counter = 0_u32;

        loop {
            interval.next().await;
            frame_counter += 1;

            // Process all pending events
            while let Some(Event { event, .. }) = gilrs.next_event() {
                match event {
                    EventType::ButtonPressed(button, _) => {
                        let message = match button {
                            Button::DPadLeft => Some(Message::PreviousGame),
                            Button::DPadRight => Some(Message::NextGame),
                            Button::LeftTrigger2 => Some(Message::PreviousSection),
                            Button::RightTrigger2 => Some(Message::NextSection),
                            Button::South => Some(Message::LaunchGame), // A button on Xbox, X on PlayStation
                            _ => None,
                        };

                        if let Some(msg) = message {
                            let _ = output.try_send(msg);
                        }
                    }
                    EventType::AxisChanged(axis, value, _) => {
                        if axis == Axis::LeftStickX {
                            left_stick_x = value;
                        }
                    }
                    _ => {}
                }
            }

            // Send repeat messages every 3 frames (~50ms) if thumbstick is held
            if frame_counter.is_multiple_of(3) {
                const AXIS_THRESHOLD: f32 = 0.5;
                let message = if left_stick_x < -AXIS_THRESHOLD {
                    Some(Message::PreviousGame)
                } else if left_stick_x > AXIS_THRESHOLD {
                    Some(Message::NextGame)
                } else {
                    None
                };

                if let Some(msg) = message {
                    let _ = output.try_send(msg);
                }
            }
        }
    })
}
