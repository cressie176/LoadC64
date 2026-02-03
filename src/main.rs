use iced::keyboard::{Key, key};
use iced::widget::{column, container, text};
use iced::{Element, Task};

mod domain;

use domain::cursor::Cursor;
use domain::game::{Game, GameId};
use domain::library::Library;
use domain::section::CharacterSection;

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
    #[allow(clippy::too_many_lines)]
    fn new() -> (Self, Task<Message>) {
        let mut library = Library::new(Box::new(CharacterSection::new));

        let games = vec![
            ("1", "Arkanoid", "arkanoid"),
            ("2", "Alien Syndrome", "alien-syndrome"),
            ("3", "Another World", "another-world"),
            ("4", "Archon", "archon"),
            ("5", "Aztec Challenge", "aztec-challenge"),
            ("6", "Boulder Dash", "boulder-dash"),
            ("7", "Bubble Bobble", "bubble-bobble"),
            ("8", "Bruce Lee", "bruce-lee"),
            ("9", "Barbarian", "barbarian"),
            ("10", "Burnin' Rubber", "burnin-rubber"),
            ("11", "Commando", "commando"),
            ("12", "Creatures", "creatures"),
            ("13", "Choplifter", "choplifter"),
            ("14", "California Games", "california-games"),
            ("15", "Cybernoid", "cybernoid"),
            ("16", "Defender of the Crown", "defender-of-the-crown"),
            ("17", "Dizzy", "dizzy"),
            ("18", "Donkey Kong", "donkey-kong"),
            ("19", "Driller", "driller"),
            ("20", "Dragon's Lair", "dragons-lair"),
            ("21", "Elite", "elite"),
            ("22", "Epyx Summer Games", "epyx-summer-games"),
            ("23", "Exolon", "exolon"),
            ("24", "Enduro Racer", "enduro-racer"),
            ("25", "Everyone's a Wally", "everyones-a-wally"),
            ("26", "Fist II", "fist-ii"),
            ("27", "Firebird", "firebird"),
            ("28", "Fort Apocalypse", "fort-apocalypse"),
            ("29", "Forbidden Forest", "forbidden-forest"),
            ("30", "Frankie Goes to Hollywood", "frankie-goes-to-hollywood"),
            ("31", "Giana Sisters", "giana-sisters"),
            ("32", "Ghosts 'n Goblins", "ghosts-n-goblins"),
            ("33", "Green Beret", "green-beret"),
            ("34", "Golden Axe", "golden-axe"),
            ("35", "Gunship", "gunship"),
            ("36", "Head over Heels", "head-over-heels"),
            ("37", "H.E.R.O.", "hero"),
            ("38", "Hysteria", "hysteria"),
            ("39", "Hawkeye", "hawkeye"),
            ("40", "Hard'n'Heavy", "hard-n-heavy"),
            ("41", "International Karate", "international-karate"),
            ("42", "Impossible Mission", "impossible-mission"),
            ("43", "IK+", "ik-plus"),
            ("44", "Infiltrator", "infiltrator"),
            ("45", "International Soccer", "international-soccer"),
            ("46", "Jet Set Willy", "jet-set-willy"),
            ("47", "Jumpman", "jumpman"),
            ("48", "Jungle Hunt", "jungle-hunt"),
            ("49", "Joust", "joust"),
            ("50", "Jupiter Lander", "jupiter-lander"),
            ("51", "Kung-Fu Master", "kung-fu-master"),
            ("52", "Katakis", "katakis"),
            ("53", "Knight Lore", "knight-lore"),
            ("54", "Kikstart", "kikstart"),
            ("55", "Krakout", "krakout"),
            ("56", "Laser Squad", "laser-squad"),
            ("57", "Last Ninja", "last-ninja"),
            ("58", "Leaderboard", "leaderboard"),
            ("59", "Lode Runner", "lode-runner"),
            ("60", "Little Computer People", "little-computer-people"),
            ("61", "Maniac Mansion", "maniac-mansion"),
            ("62", "Marble Madness", "marble-madness"),
            ("63", "Monty on the Run", "monty-on-the-run"),
            ("64", "Myth", "myth"),
            ("65", "Mega Apocalypse", "mega-apocalypse"),
            ("66", "Nebulus", "nebulus"),
            ("67", "North & South", "north-and-south"),
            ("68", "Nodes of Yesod", "nodes-of-yesod"),
            ("69", "Netherworld", "netherworld"),
            ("70", "Navy Moves", "navy-moves"),
            ("71", "Outrun", "outrun"),
            ("72", "Operation Wolf", "operation-wolf"),
            ("73", "One Man and His Droid", "one-man-and-his-droid"),
            ("74", "Olympic Games", "olympic-games"),
            ("75", "Overlander", "overlander"),
            ("76", "Paradroid", "paradroid"),
            ("77", "Platoon", "platoon"),
            ("78", "Pirates!", "pirates"),
            ("79", "Project Firestart", "project-firestart"),
            ("80", "Pitfall II", "pitfall-ii"),
            ("81", "Quake Minus One", "quake-minus-one"),
            ("82", "Quick Step", "quick-step"),
            ("83", "Quattro Adventure", "quattro-adventure"),
            ("84", "Q*bert", "qbert"),
            ("85", "Quest for Tires", "quest-for-tires"),
            ("86", "R-Type", "r-type"),
            ("87", "Rainbow Islands", "rainbow-islands"),
            ("88", "Raid over Moscow", "raid-over-moscow"),
            ("89", "Rick Dangerous", "rick-dangerous"),
            ("90", "Rambo", "rambo"),
            ("91", "Skate or Die", "skate-or-die"),
            ("92", "Summer Games II", "summer-games-ii"),
            ("93", "Supercycle", "supercycle"),
            ("94", "Sentinel", "sentinel"),
            ("95", "Spy vs Spy", "spy-vs-spy"),
            ("96", "Turrican", "turrican"),
            ("97", "The Last V8", "the-last-v8"),
            ("98", "Thing on a Spring", "thing-on-a-spring"),
            ("99", "Target Renegade", "target-renegade"),
            ("100", "Times of Lore", "times-of-lore"),
        ];

        for (id, title, sort_key) in games {
            let game = Game::new(GameId::new(id.to_string()), title.to_string(), sort_key.to_string(), None, None, None);
            library.add_game(game);
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

        container(column![carousel]).padding(20).center_x(iced::Fill).center_y(iced::Fill).into()
    }

    fn build_carousel(&self) -> Element<'_, Message> {
        if let Some(cursor) = &self.cursor {
            let games = self.library.get_game_window(cursor, -3, 7);

            if let Some(games) = games {
                let mut boxes = column![].spacing(10);

                for game in games {
                    let game_box = container(text(game.title()).size(20))
                        .padding(20)
                        .width(iced::Fill)
                        .center_x(iced::Fill)
                        .style(|_theme| container::Style {
                            border: iced::Border {
                                color: iced::Color::from_rgb(0.5, 0.5, 0.5),
                                width: 2.0,
                                radius: iced::border::Radius::from(5.0),
                            },
                            ..Default::default()
                        });
                    boxes = boxes.push(game_box);
                }

                return boxes.into();
            }
        }

        text("No games available").into()
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
