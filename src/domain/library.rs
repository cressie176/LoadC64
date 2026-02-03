use std::collections::HashMap;

use super::cursor::Cursor;
use super::game::{Game, GameId};
use super::section::Section;

type SectionFactory = Box<dyn Fn(&Game) -> Box<dyn Section>>;

const fn next_index(current: usize, len: usize) -> usize {
    (current + 1) % len
}

const fn previous_index(current: usize, len: usize) -> usize {
    (current + len - 1) % len
}

pub struct Library {
    games: HashMap<GameId, Game>,
    sections: Vec<Box<dyn Section>>,
    section_factory: SectionFactory,
}

impl Library {
    pub fn new(section_factory: SectionFactory) -> Self {
        Self {
            games: HashMap::new(),
            sections: Vec::new(),
            section_factory,
        }
    }

    pub(super) fn get_game(&self, id: &GameId) -> &Game {
        &self.games[id]
    }

    pub fn add_game(&mut self, game: Game) {
        let game_id = self.insert_game(game);
        let section_index = self.ensure_section(&game_id);
        self.categorise_game(section_index, &game_id);
    }

    fn insert_game(&mut self, game: Game) -> GameId {
        let game_id = game.id().clone();
        self.games.insert(game_id.clone(), game);
        game_id
    }

    fn ensure_section(&mut self, game_id: &GameId) -> usize {
        self.find_section(game_id).unwrap_or_else(|| self.create_section(game_id))
    }

    fn find_section(&self, game_id: &GameId) -> Option<usize> {
        self.sections.iter().position(|section| section.accepts(&self.games[game_id]))
    }

    fn create_section(&mut self, game_id: &GameId) -> usize {
        let new_section = (self.section_factory)(&self.games[game_id]);
        self.sections.push(new_section);
        self.sections.len() - 1
    }

    fn categorise_game(&mut self, section_index: usize, game_id: &GameId) {
        self.sections[section_index].add_game(&self.games[game_id], &self.games).unwrap();
    }

    pub fn next_section(&self, cursor: &Cursor) -> Option<Cursor> {
        if self.sections.is_empty() {
            return None;
        }

        let next_section = self.get_next_section(cursor);
        Cursor::first_game(next_section)
    }

    pub fn previous_section(&self, cursor: &Cursor) -> Option<Cursor> {
        if self.sections.is_empty() {
            return None;
        }

        let prev_section = self.get_previous_section(cursor);
        Cursor::first_game(prev_section)
    }

    pub fn next_game(&self, cursor: &Cursor) -> Option<Cursor> {
        if self.sections.is_empty() {
            return None;
        }
        let current_section = self.get_current_section(cursor);

        if let Some(next_game_id) = current_section.next_game_id(cursor.game_id()) {
            return Some(Cursor::for_game(current_section, next_game_id));
        }

        let next_section = self.get_next_section(cursor);
        Cursor::first_game(next_section)
    }

    pub fn previous_game(&self, cursor: &Cursor) -> Option<Cursor> {
        if self.sections.is_empty() {
            return None;
        }
        let current_section = self.get_current_section(cursor);

        if let Some(prev_game_id) = current_section.previous_game_id(cursor.game_id()) {
            return Some(Cursor::for_game(current_section, prev_game_id));
        }

        let prev_section = self.get_previous_section(cursor);
        Cursor::last_game(prev_section)
    }

    fn get_current_section(&self, cursor: &Cursor) -> &dyn Section {
        let current_section_index = self.find_section_index(cursor);
        self.sections[current_section_index].as_ref()
    }

    fn get_next_section(&self, cursor: &Cursor) -> &dyn Section {
        let current_section_index = self.find_section_index(cursor);
        let next_section_index = next_index(current_section_index, self.sections.len());
        self.sections[next_section_index].as_ref()
    }

    fn get_previous_section(&self, cursor: &Cursor) -> &dyn Section {
        let current_section_index = self.find_section_index(cursor);
        let prev_section_index = previous_index(current_section_index, self.sections.len());
        self.sections[prev_section_index].as_ref()
    }

    fn find_section_index(&self, cursor: &Cursor) -> usize {
        self.sections.iter().position(|section| section.id() == cursor.section_id()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::super::game::test_game;
    use super::super::section::{CharacterSection, SectionId};
    use super::*;

    fn create_library() -> Library {
        Library::new(Box::new(|game| Box::new(CharacterSection::new(game))))
    }

    #[test]
    fn test_add_game_to_empty_library() {
        let mut library = create_library();
        let game = test_game("1", "Monkey Island", "monkey-island");

        library.add_game(game.clone());

        assert_eq!(library.games.len(), 1);
        assert_eq!(library.sections.len(), 1);
        assert!(library.games.contains_key(game.id()));
    }

    #[test]
    fn test_add_game_to_new_section() {
        let mut library = create_library();
        let game1 = test_game("1", "Monkey Island", "monkey-island");
        let game2 = test_game("2", "Zak McKracken", "zak-mckracken");

        library.add_game(game1);
        library.add_game(game2);

        assert_eq!(library.games.len(), 2);
        assert_eq!(library.sections.len(), 2);
    }

    #[test]
    fn test_add_game_to_existing_section() {
        let mut library = create_library();
        let game1 = test_game("1", "Monkey Island", "monkey-island");
        let game2 = test_game("2", "Maniac Mansion", "maniac-mansion");

        library.add_game(game1);
        library.add_game(game2);

        assert_eq!(library.games.len(), 2);
        assert_eq!(library.sections.len(), 1);
    }

    #[test]
    fn test_add_game_with_duplicate_title() {
        let mut library = create_library();
        let game1 = test_game("1", "Monkey Island", "monkey-island");
        let game2 = test_game("2", "Monkey Island", "monkey-island");

        library.add_game(game1.clone());
        library.add_game(game2.clone());

        assert_eq!(library.games.len(), 2);
        assert!(library.games.contains_key(game1.id()));
        assert!(library.games.contains_key(game2.id()));
    }

    #[test]
    fn test_next_section_empty_library() {
        let library = create_library();
        let cursor = Cursor::new(SectionId::new(), GameId::new("1".to_string()));

        let result = library.next_section(&cursor);

        assert!(result.is_none());
    }

    #[test]
    fn test_next_section_single_section() {
        let mut library = create_library();
        let game1 = test_game("1", "Monkey Island", "monkey-island");
        let game2 = test_game("2", "Maniac Mansion", "maniac-mansion");

        library.add_game(game1.clone());
        library.add_game(game2.clone());

        let section_id = library.sections[0].id().clone();
        let cursor = Cursor::new(section_id, game1.id().clone());

        let result = library.next_section(&cursor);

        assert!(result.is_some());
        let next_cursor = result.unwrap();
        assert_eq!(next_cursor.section_id(), library.sections[0].id());
        assert_eq!(next_cursor.game_id(), game2.id());
    }

    #[test]
    fn test_next_section_two_sections_no_wrap() {
        let mut library = create_library();
        let game1 = test_game("1", "Alice in Wonderland", "alice-in-wonderland");
        let game2 = test_game("2", "Another World", "another-world");
        let game3 = test_game("3", "Bubble Bobble", "bubble-bobble");
        let game4 = test_game("4", "Boulder Dash", "boulder-dash");

        library.add_game(game1.clone());
        library.add_game(game2.clone());
        library.add_game(game3.clone());
        library.add_game(game4.clone());

        let section_id = library.sections[0].id().clone();
        let cursor = Cursor::new(section_id, game1.id().clone());

        let result = library.next_section(&cursor);

        assert!(result.is_some());
        let next_cursor = result.unwrap();
        assert_eq!(next_cursor.section_id(), library.sections[1].id());
        assert_eq!(next_cursor.game_id(), game4.id());
    }

    #[test]
    fn test_next_section_two_sections_with_wrap() {
        let mut library = create_library();
        let game1 = test_game("1", "Alice in Wonderland", "alice-in-wonderland");
        let game2 = test_game("2", "Another World", "another-world");
        let game3 = test_game("3", "Bubble Bobble", "bubble-bobble");
        let game4 = test_game("4", "Boulder Dash", "boulder-dash");

        library.add_game(game1.clone());
        library.add_game(game2.clone());
        library.add_game(game3.clone());
        library.add_game(game4.clone());

        let section_id = library.sections[1].id().clone();
        let cursor = Cursor::new(section_id, game3.id().clone());

        let result = library.next_section(&cursor);

        assert!(result.is_some());
        let next_cursor = result.unwrap();
        assert_eq!(next_cursor.section_id(), library.sections[0].id());
        assert_eq!(next_cursor.game_id(), game1.id());
    }

    #[test]
    fn test_previous_section_empty_library() {
        let library = create_library();
        let cursor = Cursor::new(SectionId::new(), GameId::new("1".to_string()));

        let result = library.previous_section(&cursor);

        assert!(result.is_none());
    }

    #[test]
    fn test_previous_section_single_section() {
        let mut library = create_library();
        let game1 = test_game("1", "Monkey Island", "monkey-island");
        let game2 = test_game("2", "Maniac Mansion", "maniac-mansion");

        library.add_game(game1.clone());
        library.add_game(game2.clone());

        let section_id = library.sections[0].id().clone();
        let cursor = Cursor::new(section_id, game1.id().clone());

        let result = library.previous_section(&cursor);

        assert!(result.is_some());
        let prev_cursor = result.unwrap();
        assert_eq!(prev_cursor.section_id(), library.sections[0].id());
        assert_eq!(prev_cursor.game_id(), game2.id());
    }

    #[test]
    fn test_previous_section_two_sections_no_wrap() {
        let mut library = create_library();
        let game1 = test_game("1", "Alice in Wonderland", "alice-in-wonderland");
        let game2 = test_game("2", "Another World", "another-world");
        let game3 = test_game("3", "Bubble Bobble", "bubble-bobble");
        let game4 = test_game("4", "Boulder Dash", "boulder-dash");

        library.add_game(game1.clone());
        library.add_game(game2.clone());
        library.add_game(game3.clone());
        library.add_game(game4.clone());

        let section_id = library.sections[1].id().clone();
        let cursor = Cursor::new(section_id, game3.id().clone());

        let result = library.previous_section(&cursor);

        assert!(result.is_some());
        let prev_cursor = result.unwrap();
        assert_eq!(prev_cursor.section_id(), library.sections[0].id());
        assert_eq!(prev_cursor.game_id(), game1.id());
    }

    #[test]
    fn test_previous_section_two_sections_with_wrap() {
        let mut library = create_library();
        let game1 = test_game("1", "Alice in Wonderland", "alice-in-wonderland");
        let game2 = test_game("2", "Another World", "another-world");
        let game3 = test_game("3", "Bubble Bobble", "bubble-bobble");
        let game4 = test_game("4", "Boulder Dash", "boulder-dash");

        library.add_game(game1.clone());
        library.add_game(game2.clone());
        library.add_game(game3.clone());
        library.add_game(game4.clone());

        let section_id = library.sections[0].id().clone();
        let cursor = Cursor::new(section_id, game1.id().clone());

        let result = library.previous_section(&cursor);

        assert!(result.is_some());
        let prev_cursor = result.unwrap();
        assert_eq!(prev_cursor.section_id(), library.sections[1].id());
        assert_eq!(prev_cursor.game_id(), game4.id());
    }

    #[test]
    fn test_next_game_empty_library() {
        let library = create_library();
        let cursor = Cursor::new(SectionId::new(), GameId::new("1".to_string()));

        let result = library.next_game(&cursor);

        assert!(result.is_none());
    }

    #[test]
    fn test_next_game_single_game() {
        let mut library = create_library();
        let game = test_game("1", "Monkey Island", "monkey-island");

        library.add_game(game.clone());

        let section_id = library.sections[0].id().clone();
        let cursor = Cursor::new(section_id, game.id().clone());

        let result = library.next_game(&cursor);

        assert!(result.is_some());
        let next_cursor = result.unwrap();
        assert_eq!(next_cursor.section_id(), library.sections[0].id());
        assert_eq!(next_cursor.game_id(), game.id());
    }

    #[test]
    fn test_next_game_first_of_two_in_section() {
        let mut library = create_library();
        let game1 = test_game("1", "Maniac Mansion", "maniac-mansion");
        let game2 = test_game("2", "Monkey Island", "monkey-island");

        library.add_game(game1.clone());
        library.add_game(game2.clone());

        let section_id = library.sections[0].id().clone();
        let cursor = Cursor::new(section_id, game1.id().clone());

        let result = library.next_game(&cursor);

        assert!(result.is_some());
        let next_cursor = result.unwrap();
        assert_eq!(next_cursor.section_id(), library.sections[0].id());
        assert_eq!(next_cursor.game_id(), game2.id());
    }

    #[test]
    fn test_next_game_last_in_single_section() {
        let mut library = create_library();
        let game1 = test_game("1", "Maniac Mansion", "maniac-mansion");
        let game2 = test_game("2", "Monkey Island", "monkey-island");

        library.add_game(game1.clone());
        library.add_game(game2.clone());

        let section_id = library.sections[0].id().clone();
        let cursor = Cursor::new(section_id, game2.id().clone());

        let result = library.next_game(&cursor);

        assert!(result.is_some());
        let next_cursor = result.unwrap();
        assert_eq!(next_cursor.section_id(), library.sections[0].id());
        assert_eq!(next_cursor.game_id(), game1.id());
    }

    #[test]
    fn test_next_game_last_in_section_with_next_section() {
        let mut library = create_library();
        let game1 = test_game("1", "Alice in Wonderland", "alice-in-wonderland");
        let game2 = test_game("2", "Another World", "another-world");
        let game3 = test_game("3", "Bubble Bobble", "bubble-bobble");
        let game4 = test_game("4", "Boulder Dash", "boulder-dash");

        library.add_game(game1.clone());
        library.add_game(game2.clone());
        library.add_game(game3.clone());
        library.add_game(game4.clone());

        let section_id = library.sections[0].id().clone();
        let cursor = Cursor::new(section_id, game2.id().clone());

        let result = library.next_game(&cursor);

        assert!(result.is_some());
        let next_cursor = result.unwrap();
        assert_eq!(next_cursor.section_id(), library.sections[1].id());
        assert_eq!(next_cursor.game_id(), game4.id());
    }

    #[test]
    fn test_next_game_last_in_last_section_wraps() {
        let mut library = create_library();
        let game1 = test_game("1", "Alice in Wonderland", "alice-in-wonderland");
        let game2 = test_game("2", "Another World", "another-world");
        let game3 = test_game("3", "Bubble Bobble", "bubble-bobble");
        let game4 = test_game("4", "Boulder Dash", "boulder-dash");

        library.add_game(game1.clone());
        library.add_game(game2.clone());
        library.add_game(game3.clone());
        library.add_game(game4.clone());

        let section_id = library.sections[1].id().clone();
        let cursor = Cursor::new(section_id, game3.id().clone());

        let result = library.next_game(&cursor);

        assert!(result.is_some());
        let next_cursor = result.unwrap();
        assert_eq!(next_cursor.section_id(), library.sections[0].id());
        assert_eq!(next_cursor.game_id(), game1.id());
    }

    #[test]
    fn test_previous_game_empty_library() {
        let library = create_library();
        let cursor = Cursor::new(SectionId::new(), GameId::new("1".to_string()));

        let result = library.previous_game(&cursor);

        assert!(result.is_none());
    }

    #[test]
    fn test_previous_game_single_game() {
        let mut library = create_library();
        let game = test_game("1", "Monkey Island", "monkey-island");

        library.add_game(game.clone());

        let section_id = library.sections[0].id().clone();
        let cursor = Cursor::new(section_id, game.id().clone());

        let result = library.previous_game(&cursor);

        assert!(result.is_some());
        let prev_cursor = result.unwrap();
        assert_eq!(prev_cursor.section_id(), library.sections[0].id());
        assert_eq!(prev_cursor.game_id(), game.id());
    }

    #[test]
    fn test_previous_game_second_of_two_in_section() {
        let mut library = create_library();
        let game1 = test_game("1", "Maniac Mansion", "maniac-mansion");
        let game2 = test_game("2", "Monkey Island", "monkey-island");

        library.add_game(game1.clone());
        library.add_game(game2.clone());

        let section_id = library.sections[0].id().clone();
        let cursor = Cursor::new(section_id, game2.id().clone());

        let result = library.previous_game(&cursor);

        assert!(result.is_some());
        let prev_cursor = result.unwrap();
        assert_eq!(prev_cursor.section_id(), library.sections[0].id());
        assert_eq!(prev_cursor.game_id(), game1.id());
    }

    #[test]
    fn test_previous_game_first_in_single_section() {
        let mut library = create_library();
        let game1 = test_game("1", "Maniac Mansion", "maniac-mansion");
        let game2 = test_game("2", "Monkey Island", "monkey-island");

        library.add_game(game1.clone());
        library.add_game(game2.clone());

        let section_id = library.sections[0].id().clone();
        let cursor = Cursor::new(section_id, game1.id().clone());

        let result = library.previous_game(&cursor);

        assert!(result.is_some());
        let prev_cursor = result.unwrap();
        assert_eq!(prev_cursor.section_id(), library.sections[0].id());
        assert_eq!(prev_cursor.game_id(), game2.id());
    }

    #[test]
    fn test_previous_game_first_in_section_with_previous_section() {
        let mut library = create_library();
        let game1 = test_game("1", "Alice in Wonderland", "alice-in-wonderland");
        let game2 = test_game("2", "Another World", "another-world");
        let game3 = test_game("3", "Bubble Bobble", "bubble-bobble");
        let game4 = test_game("4", "Boulder Dash", "boulder-dash");

        library.add_game(game1.clone());
        library.add_game(game2.clone());
        library.add_game(game3.clone());
        library.add_game(game4.clone());

        let section_id = library.sections[1].id().clone();
        let cursor = Cursor::new(section_id, game4.id().clone());

        let result = library.previous_game(&cursor);

        assert!(result.is_some());
        let prev_cursor = result.unwrap();
        assert_eq!(prev_cursor.section_id(), library.sections[0].id());
        assert_eq!(prev_cursor.game_id(), game2.id());
    }

    #[test]
    fn test_previous_game_first_in_first_section_wraps() {
        let mut library = create_library();
        let game1 = test_game("1", "Alice in Wonderland", "alice-in-wonderland");
        let game2 = test_game("2", "Another World", "another-world");
        let game3 = test_game("3", "Bubble Bobble", "bubble-bobble");
        let game4 = test_game("4", "Boulder Dash", "boulder-dash");

        library.add_game(game1.clone());
        library.add_game(game2.clone());
        library.add_game(game3.clone());
        library.add_game(game4.clone());

        let section_id = library.sections[0].id().clone();
        let cursor = Cursor::new(section_id, game1.id().clone());

        let result = library.previous_game(&cursor);

        assert!(result.is_some());
        let prev_cursor = result.unwrap();
        assert_eq!(prev_cursor.section_id(), library.sections[1].id());
        assert_eq!(prev_cursor.game_id(), game3.id());
    }
}
