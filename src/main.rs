use iced::widget::{Stack, column, container, row, text};
use iced::{Element, Task};
use std::path::PathBuf;

mod cli;
mod domain;
mod infrastructure;
mod ui;

use domain::cursor::Cursor;
use domain::game::Game;
use domain::library::Library;
use domain::section::CharacterSection;
use infrastructure::{game_loader, vice_emulator::ViceEmulator};
use ui::{carousel, carousel_layout::CarouselLayout, game_info, input, theme};

const DEFAULT_WINDOW_WIDTH: f32 = 1280.0;

fn main() -> iced::Result {
    iced::application("Load!64", App::update, App::view).subscription(App::subscription).run_with(App::new)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Browse,
    Manage,
}

struct App {
    library: Library<CharacterSection>,
    cursor: Option<Cursor>,
    window_width: f32,
    vice_emulator: ViceEmulator,
    mode: Mode,
    all_games: Vec<Game>,
    games_dir: PathBuf,
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
    ToggleMode,
    HideGame,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let args = cli::parse();

        let all_games = game_loader::load_all_games(&args.games_dir).expect("Error loading games");

        let mut library = Library::new(Box::new(CharacterSection::new));
        for game in &all_games {
            if !game.is_hidden() {
                library.add_game(game.clone());
            }
        }

        let cursor = library.get_cursor();
        let vice_emulator = ViceEmulator::new(args.vice_path);

        (Self { library, cursor, window_width: DEFAULT_WINDOW_WIDTH, vice_emulator, mode: Mode::Browse, all_games, games_dir: args.games_dir }, Task::none())
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::WindowResized(width, _height) => {
                self.window_width = width;
            }
            Message::NextGame => {
                self.cursor = self.cursor.as_ref().and_then(|cursor| self.library.next_game(cursor));
            }
            Message::PreviousGame => {
                self.cursor = self.cursor.as_ref().and_then(|cursor| self.library.previous_game(cursor));
            }
            Message::NextSection => {
                self.cursor = self.cursor.as_ref().and_then(|cursor| self.library.next_section(cursor));
            }
            Message::PreviousSection => {
                self.cursor = self.cursor.as_ref().and_then(|cursor| self.library.previous_section(cursor));
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
            Message::ToggleMode => {
                self.mode = match self.mode {
                    Mode::Browse => Mode::Manage,
                    Mode::Manage => Mode::Browse,
                };
                self.rebuild_library();
            }
            Message::HideGame => {
                if self.mode != Mode::Manage {
                    return;
                }

                if let Some(cursor) = &self.cursor {
                    let game = self.library.get_game(cursor);
                    let game_id = game.id().clone();

                    if let Some(game) = self.all_games.iter_mut().find(|g| g.id() == &game_id) {
                        game.set_hidden(!game.is_hidden());

                        if let Err(e) = game_loader::save_game_config(game) {
                            eprintln!("Failed to save config: {e}");
                        }

                        self.rebuild_library();
                    }
                }
            }
        }
    }

    fn rebuild_library(&mut self) {
        let include_hidden = self.mode == Mode::Manage;
        let current_game_id = self.cursor.as_ref().map(|cursor| self.library.get_game(cursor).id().clone());

        let mut new_library = Library::new(Box::new(CharacterSection::new));

        for game in &self.all_games {
            if include_hidden || !game.is_hidden() {
                new_library.add_game(game.clone());
            }
        }

        self.library = new_library;

        if let Some(game_id) = current_game_id {
            self.cursor = self.library.to_game(&game_id).or_else(|| self.library.get_cursor());
        } else {
            self.cursor = self.library.get_cursor();
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

        self.vice_emulator.launch(&self.games_dir, rom.path()).expect("Failed to launch VICE");
    }

    fn view(&self) -> Element<'_, Message> {
        let layout = CarouselLayout::new(self.window_width);
        let (carousel_games, info) = self.get_carousel_games(&layout);
        let carousel = carousel::create_carousel_container(carousel_games, &layout);

        let content = column![carousel, info].spacing(theme::CONTENT_SPACING);

        let centered_content = container(content).center_x(iced::Fill).center_y(iced::Fill);

        let view: Element<'_, Message> = if self.mode == Mode::Manage {
            let mode_text = text("Management Mode").size(theme::METADATA_FONT_SIZE).color(theme::TEXT_COLOR);
            let mode_indicator = container(mode_text).padding(10).align_x(iced::alignment::Horizontal::Right).width(iced::Fill);

            Stack::new().push(centered_content).push(mode_indicator).into()
        } else {
            centered_content.into()
        };

        container(view).center_x(iced::Fill).style(|_theme| container::Style { background: Some(iced::Background::Color(theme::BACKGROUND_COLOR)), ..Default::default() }).into()
    }

    fn get_carousel_games(&self, layout: &CarouselLayout) -> (iced::widget::Row<'_, Message>, Element<'_, Message>) {
        let mut info: Element<'_, Message> = container(text("")).into();

        #[allow(clippy::option_if_let_else)]
        let carousel_games = if let Some(cursor) = &self.cursor {
            if let Some(games) = self.library.get_game_window(cursor, layout.offset(), layout.number_of_games()) {
                if let Some(current_game) = games.get(layout.current_game_index()) {
                    info = game_info::create_game_info(current_game);
                }
                carousel::create_carousel_row(&games, layout, self.mode)
            } else {
                row![].spacing(CarouselLayout::spacing()).align_y(iced::Alignment::Center)
            }
        } else {
            row![].spacing(CarouselLayout::spacing()).align_y(iced::Alignment::Center)
        };

        (carousel_games, info)
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
            iced::keyboard::Key::Named(iced::keyboard::key::Named::Tab) => Some(Message::ToggleMode),
            iced::keyboard::Key::Named(iced::keyboard::key::Named::Backspace) => Some(Message::HideGame),
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
