use super::game::Game;
use super::section::Section;
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
pub struct CharacterSection {
    character: char,
    title: String,
    games: Vec<Game>,
    current_game_idx: usize,
}

impl CharacterSection {
    pub fn new(character: char) -> Self {
        Self {
            character,
            title: character.to_uppercase().to_string(),
            games: Vec::new(),
            current_game_idx: 0,
        }
    }
}

impl Section for CharacterSection {
    fn add(&mut self, game: &Game) -> bool {
        if !game.starts_with(self.character) {
            return false;
        }
        self.games.push(game.clone());
        self.games.sort();
        true
    }

    fn next(&mut self) -> bool {
        if !self.can_move_forward() {
            return false;
        }
        self.increment_current_game_idx();
        true
    }

    fn previous(&mut self) -> bool {
        if !self.can_move_backward() {
            return false;
        }
        self.decrement_current_game_idx();
        true
    }

    fn first(&mut self) {
        self.current_game_idx = 0;
    }

    fn last(&mut self) {
        if !self.games.is_empty() {
            self.current_game_idx = self.games.len() - 1;
        }
    }

    fn with_current_game<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&Game) -> R,
    {
        if self.games.is_empty() {
            return None;
        }
        Some(f(&self.games[self.current_game_idx]))
    }
}

impl CharacterSection {
    const fn can_move_forward(&self) -> bool {
        self.current_game_idx + 1 < self.games.len()
    }

    const fn can_move_backward(&self) -> bool {
        self.current_game_idx > 0
    }

    const fn increment_current_game_idx(&mut self) {
        self.current_game_idx = (self.current_game_idx + 1) % self.games.len();
    }

    const fn decrement_current_game_idx(&mut self) {
        self.current_game_idx = (self.current_game_idx + self.games.len() - 1) % self.games.len();
    }
}

impl Ord for CharacterSection {
    fn cmp(&self, other: &Self) -> Ordering {
        self.character.cmp(&other.character)
    }
}

impl PartialOrd for CharacterSection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_section_adds_matching_games() {
        let mut section = CharacterSection::new('m');
        let game = Game::new(
            "Monkey Island".to_string(),
            "monkey-island".to_string(),
            None,
            None,
            None,
        );

        assert!(section.add(&game));
    }

    #[test]
    fn test_character_section_rejects_non_matching_games() {
        let mut section = CharacterSection::new('m');
        let game = Game::new(
            "Zak McKracken".to_string(),
            "zak-mckracken".to_string(),
            None,
            None,
            None,
        );

        assert!(!section.add(&game));
    }

    #[test]
    fn test_character_section_sorts_games() {
        let mut section = CharacterSection::new('m');

        let game1 = Game::new(
            "Monkey Island 2".to_string(),
            "monkey-island-2".to_string(),
            None,
            None,
            None,
        );
        let game2 = Game::new(
            "Maniac Mansion".to_string(),
            "maniac-mansion".to_string(),
            None,
            None,
            None,
        );
        let game3 = Game::new(
            "Monkey Island".to_string(),
            "monkey-island".to_string(),
            None,
            None,
            None,
        );

        section.add(&game1);
        section.add(&game2);
        section.add(&game3);

        assert_eq!(section.games.len(), 3);
        assert_eq!(section.games[0].starts_with('m'), true);
    }

    #[test]
    fn test_character_section_next_navigation() {
        let mut section = CharacterSection::new('m');

        let game1 = Game::new(
            "Maniac Mansion".to_string(),
            "maniac-mansion".to_string(),
            None,
            None,
            None,
        );
        let game2 = Game::new(
            "Monkey Island".to_string(),
            "monkey-island".to_string(),
            None,
            None,
            None,
        );
        let game3 = Game::new(
            "Marble Madness".to_string(),
            "marble-madness".to_string(),
            None,
            None,
            None,
        );

        section.add(&game1);
        section.add(&game2);
        section.add(&game3);

        let title =
            section.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Maniac Mansion".to_string()));

        let moved = section.next();
        assert!(moved);
        let title =
            section.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Marble Madness".to_string()));

        let moved = section.next();
        assert!(moved);
        let title =
            section.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Monkey Island".to_string()));

        let moved = section.next();
        assert!(!moved);
        let title =
            section.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Monkey Island".to_string()));
    }

    #[test]
    fn test_character_section_previous_navigation() {
        let mut section = CharacterSection::new('m');

        let game1 = Game::new(
            "Maniac Mansion".to_string(),
            "maniac-mansion".to_string(),
            None,
            None,
            None,
        );
        let game2 = Game::new(
            "Monkey Island".to_string(),
            "monkey-island".to_string(),
            None,
            None,
            None,
        );
        let game3 = Game::new(
            "Marble Madness".to_string(),
            "marble-madness".to_string(),
            None,
            None,
            None,
        );

        section.add(&game1);
        section.add(&game2);
        section.add(&game3);

        let title =
            section.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Maniac Mansion".to_string()));

        let moved = section.previous();
        assert!(!moved);
        let title =
            section.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Maniac Mansion".to_string()));
    }

    #[test]
    fn test_character_section_navigation_empty() {
        let mut section = CharacterSection::new('m');

        section.next();
        section.previous();

        assert_eq!(section.current_game_idx, 0);
    }
}
