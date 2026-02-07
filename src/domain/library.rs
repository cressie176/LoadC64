use std::collections::HashMap;

use super::cursor::Cursor;
use super::game::{Game, GameId};
use super::section::Section;

const fn next_index(current: usize, len: usize) -> usize {
    (current + 1) % len
}

const fn previous_index(current: usize, len: usize) -> usize {
    (current + len - 1) % len
}

pub struct Library<S: Section + Ord> {
    games: HashMap<GameId, Game>,
    sections: Vec<S>,
    section_factory: Box<dyn Fn(&Game) -> S>,
}

impl<S: Section + Ord> Library<S> {
    pub fn new(section_factory: Box<dyn Fn(&Game) -> S>) -> Self {
        Self { games: HashMap::new(), sections: Vec::new(), section_factory }
    }

    pub fn get_cursor(&self) -> Option<Cursor> {
        let first_section = self.sections.first()?;
        Cursor::first_game(first_section)
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
        self.sections.sort();
        self.find_section(game_id).unwrap()
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

    pub fn to_section(&self, value: &str) -> Option<Cursor> {
        let section = self.sections.iter().find(|s| s.satisfies(value))?;
        Cursor::first_game(section)
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

    pub fn to_game(&self, game_id: &GameId) -> Option<Cursor> {
        let game = self.games.get(game_id)?;
        let section = self.sections.iter().find(|s| s.accepts(game))?;
        Some(Cursor::for_game(section, game_id))
    }

    pub fn get_game(&self, cursor: &Cursor) -> Option<&Game> {
        self.games.get(cursor.game_id())
    }

    fn get_game_by_id(&self, id: &GameId) -> &Game {
        &self.games[id]
    }

    pub fn get_game_window(&self, cursor: &Cursor, offset: i32, count: usize) -> Option<Vec<&Game>> {
        if self.sections.is_empty() {
            return None;
        }
        let start_cursor = self.iterate_backwards(cursor, offset.abs(), |_| {})?;

        let mut games = Vec::with_capacity(count);
        games.push(self.get_game_by_id(start_cursor.game_id()));

        self.iterate_forwards(&start_cursor, count - 1, |game_id| {
            games.push(self.get_game_by_id(game_id));
        });

        Some(games)
    }

    fn iterate_forwards<F>(&self, cursor: &Cursor, steps: usize, mut callback: F)
    where
        F: FnMut(&GameId),
    {
        let mut current_cursor = cursor.clone();
        for _ in 0..steps {
            current_cursor = self.next_game(&current_cursor).unwrap();
            callback(current_cursor.game_id());
        }
    }

    fn iterate_backwards<F>(&self, cursor: &Cursor, steps: i32, mut callback: F) -> Option<Cursor>
    where
        F: FnMut(&GameId),
    {
        let mut current_cursor = cursor.clone();
        for _ in 0..steps {
            current_cursor = self.previous_game(&current_cursor)?;
            callback(current_cursor.game_id());
        }
        Some(current_cursor)
    }

    fn get_current_section(&self, cursor: &Cursor) -> &S {
        let current_section_index = self.find_section_index(cursor);
        &self.sections[current_section_index]
    }

    fn get_next_section(&self, cursor: &Cursor) -> &S {
        let current_section_index = self.find_section_index(cursor);
        let next_section_index = next_index(current_section_index, self.sections.len());
        &self.sections[next_section_index]
    }

    fn get_previous_section(&self, cursor: &Cursor) -> &S {
        let current_section_index = self.find_section_index(cursor);
        let prev_section_index = previous_index(current_section_index, self.sections.len());
        &self.sections[prev_section_index]
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

    fn create_library() -> Library<CharacterSection> {
        Library::new(Box::new(|game| CharacterSection::new(game)))
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

    #[test]
    fn test_get_game_window_empty_library() {
        let library = create_library();
        let cursor = Cursor::new(SectionId::new(), GameId::new("1".to_string()));

        let window = library.get_game_window(&cursor, -1, 3);

        assert!(window.is_none());
    }

    #[test]
    fn test_get_game_window_single_game() {
        let mut library = create_library();
        let game1 = test_game("1", "Monkey Island", "monkey-island");

        library.add_game(game1.clone());

        let section_id = library.sections[0].id().clone();
        let cursor = Cursor::new(section_id, game1.id().clone());

        let window = library.get_game_window(&cursor, -1, 3).unwrap();

        assert_eq!(window.len(), 3);
        assert_eq!(window[0].id(), game1.id());
        assert_eq!(window[1].id(), game1.id());
        assert_eq!(window[2].id(), game1.id());
    }

    #[test]
    fn test_get_game_window_two_games_first() {
        let mut library = create_library();
        let game1 = test_game("1", "Maniac Mansion", "maniac-mansion");
        let game2 = test_game("2", "Monkey Island", "monkey-island");

        library.add_game(game1.clone());
        library.add_game(game2.clone());

        let section_id = library.sections[0].id().clone();
        let cursor = Cursor::new(section_id, game1.id().clone());

        let window = library.get_game_window(&cursor, -1, 3).unwrap();

        assert_eq!(window.len(), 3);
        assert_eq!(window[0].id(), game2.id());
        assert_eq!(window[1].id(), game1.id());
        assert_eq!(window[2].id(), game2.id());
    }

    #[test]
    fn test_get_game_window_two_games_second() {
        let mut library = create_library();
        let game1 = test_game("1", "Maniac Mansion", "maniac-mansion");
        let game2 = test_game("2", "Monkey Island", "monkey-island");

        library.add_game(game1.clone());
        library.add_game(game2.clone());

        let section_id = library.sections[0].id().clone();
        let cursor = Cursor::new(section_id, game2.id().clone());

        let window = library.get_game_window(&cursor, -1, 3).unwrap();

        assert_eq!(window.len(), 3);
        assert_eq!(window[0].id(), game1.id());
        assert_eq!(window[1].id(), game2.id());
        assert_eq!(window[2].id(), game1.id());
    }

    #[test]
    fn test_get_game_window_five_games_first() {
        let mut library = create_library();
        let game1 = test_game("1", "Game 1", "game-1");
        let game2 = test_game("2", "Game 2", "game-2");
        let game3 = test_game("3", "Game 3", "game-3");
        let game4 = test_game("4", "Game 4", "game-4");
        let game5 = test_game("5", "Game 5", "game-5");

        library.add_game(game1.clone());
        library.add_game(game2.clone());
        library.add_game(game3.clone());
        library.add_game(game4.clone());
        library.add_game(game5.clone());

        let section_id = library.sections[0].id().clone();
        let cursor = Cursor::new(section_id, game1.id().clone());

        let window = library.get_game_window(&cursor, -1, 3).unwrap();

        assert_eq!(window.len(), 3);
        assert_eq!(window[0].id(), game5.id());
        assert_eq!(window[1].id(), game1.id());
        assert_eq!(window[2].id(), game2.id());
    }

    #[test]
    fn test_get_game_window_five_games_third() {
        let mut library = create_library();
        let game1 = test_game("1", "Game 1", "game-1");
        let game2 = test_game("2", "Game 2", "game-2");
        let game3 = test_game("3", "Game 3", "game-3");
        let game4 = test_game("4", "Game 4", "game-4");
        let game5 = test_game("5", "Game 5", "game-5");

        library.add_game(game1.clone());
        library.add_game(game2.clone());
        library.add_game(game3.clone());
        library.add_game(game4.clone());
        library.add_game(game5.clone());

        let section_id = library.sections[0].id().clone();
        let cursor = Cursor::new(section_id, game3.id().clone());

        let window = library.get_game_window(&cursor, -1, 3).unwrap();

        assert_eq!(window.len(), 3);
        assert_eq!(window[0].id(), game2.id());
        assert_eq!(window[1].id(), game3.id());
        assert_eq!(window[2].id(), game4.id());
    }

    #[test]
    fn test_get_game_window_five_games_fifth() {
        let mut library = create_library();
        let game1 = test_game("1", "Game 1", "game-1");
        let game2 = test_game("2", "Game 2", "game-2");
        let game3 = test_game("3", "Game 3", "game-3");
        let game4 = test_game("4", "Game 4", "game-4");
        let game5 = test_game("5", "Game 5", "game-5");

        library.add_game(game1.clone());
        library.add_game(game2.clone());
        library.add_game(game3.clone());
        library.add_game(game4.clone());
        library.add_game(game5.clone());

        let section_id = library.sections[0].id().clone();
        let cursor = Cursor::new(section_id, game5.id().clone());

        let window = library.get_game_window(&cursor, -1, 3).unwrap();

        assert_eq!(window.len(), 3);
        assert_eq!(window[0].id(), game4.id());
        assert_eq!(window[1].id(), game5.id());
        assert_eq!(window[2].id(), game1.id());
    }

    #[test]
    fn test_get_game_window_two_sections_first_first() {
        let mut library = create_library();
        let game1 = test_game("1", "Alice", "alice");
        let game2 = test_game("2", "Another", "another");
        let game3 = test_game("3", "Bubble", "bubble");
        let game4 = test_game("4", "Boulder", "boulder");

        library.add_game(game1.clone());
        library.add_game(game2.clone());
        library.add_game(game3.clone());
        library.add_game(game4.clone());

        let section_id = library.sections[0].id().clone();
        let cursor = Cursor::new(section_id, game1.id().clone());

        let window = library.get_game_window(&cursor, -1, 3).unwrap();

        assert_eq!(window.len(), 3);
        assert_eq!(window[0].id(), game3.id());
        assert_eq!(window[1].id(), game1.id());
        assert_eq!(window[2].id(), game2.id());
    }

    #[test]
    fn test_get_game_window_two_sections_first_second() {
        let mut library = create_library();
        let game1 = test_game("1", "Alice", "alice");
        let game2 = test_game("2", "Another", "another");
        let game3 = test_game("3", "Bubble", "bubble");
        let game4 = test_game("4", "Boulder", "boulder");

        library.add_game(game1.clone());
        library.add_game(game2.clone());
        library.add_game(game3.clone());
        library.add_game(game4.clone());

        let section_id = library.sections[0].id().clone();
        let cursor = Cursor::new(section_id, game2.id().clone());

        let window = library.get_game_window(&cursor, -1, 3).unwrap();

        assert_eq!(window.len(), 3);
        assert_eq!(window[0].id(), game1.id());
        assert_eq!(window[1].id(), game2.id());
        assert_eq!(window[2].id(), game4.id());
    }

    #[test]
    fn test_get_game_window_two_sections_second_first() {
        let mut library = create_library();
        let game1 = test_game("1", "Alice", "alice");
        let game2 = test_game("2", "Another", "another");
        let game3 = test_game("3", "Bubble", "bubble");
        let game4 = test_game("4", "Boulder", "boulder");

        library.add_game(game1.clone());
        library.add_game(game2.clone());
        library.add_game(game3.clone());
        library.add_game(game4.clone());

        let section_id = library.sections[1].id().clone();
        let cursor = Cursor::new(section_id, game4.id().clone());

        let window = library.get_game_window(&cursor, -1, 3).unwrap();

        assert_eq!(window.len(), 3);
        assert_eq!(window[0].id(), game2.id());
        assert_eq!(window[1].id(), game4.id());
        assert_eq!(window[2].id(), game3.id());
    }

    #[test]
    fn test_get_game_window_two_sections_second_second() {
        let mut library = create_library();
        let game1 = test_game("1", "Alice", "alice");
        let game2 = test_game("2", "Another", "another");
        let game3 = test_game("3", "Bubble", "bubble");
        let game4 = test_game("4", "Boulder", "boulder");

        library.add_game(game1.clone());
        library.add_game(game2.clone());
        library.add_game(game3.clone());
        library.add_game(game4.clone());

        let section_id = library.sections[1].id().clone();
        let cursor = Cursor::new(section_id, game3.id().clone());

        let window = library.get_game_window(&cursor, -1, 3).unwrap();

        assert_eq!(window.len(), 3);
        assert_eq!(window[0].id(), game4.id());
        assert_eq!(window[1].id(), game3.id());
        assert_eq!(window[2].id(), game1.id());
    }

    #[test]
    fn test_sections_sorted_after_creation() {
        let mut library = create_library();
        let game_m = test_game("1", "Monkey Island", "monkey-island");
        let game_z = test_game("2", "Zak McKracken", "zak-mckracken");
        let game_a = test_game("3", "Another World", "another-world");

        library.add_game(game_m.clone());
        library.add_game(game_z.clone());
        library.add_game(game_a.clone());

        assert_eq!(library.sections.len(), 3);
        assert_eq!(library.sections[0].title(), "Section 'A'");
        assert_eq!(library.sections[1].title(), "Section 'M'");
        assert_eq!(library.sections[2].title(), "Section 'Z'");

        let cursor = Cursor::new(library.sections[0].id().clone(), game_a.id().clone());
        let next = library.next_section(&cursor).unwrap();
        assert_eq!(next.section_id(), library.sections[1].id());
        assert_eq!(next.game_id(), game_m.id());

        let next2 = library.next_section(&next).unwrap();
        assert_eq!(next2.section_id(), library.sections[2].id());
        assert_eq!(next2.game_id(), game_z.id());
    }

    #[test]
    fn test_to_section_finds_existing_section() {
        let mut library = create_library();
        let game1 = test_game("1", "Alice", "alice");
        let game2 = test_game("2", "Another", "another");
        let game3 = test_game("3", "Monkey Island", "monkey-island");

        library.add_game(game1.clone());
        library.add_game(game2.clone());
        library.add_game(game3.clone());

        let cursor = library.to_section("M").unwrap();

        assert_eq!(cursor.section_id(), library.sections[1].id());
        assert_eq!(cursor.game_id(), game3.id());
    }

    #[test]
    fn test_to_section_case_insensitive() {
        let mut library = create_library();
        let game = test_game("1", "Monkey Island", "monkey-island");

        library.add_game(game.clone());

        let cursor_upper = library.to_section("M");
        let cursor_lower = library.to_section("m");

        assert!(cursor_upper.is_some());
        assert!(cursor_lower.is_some());
        assert_eq!(cursor_upper.unwrap().game_id(), game.id());
        assert_eq!(cursor_lower.unwrap().game_id(), game.id());
    }

    #[test]
    fn test_to_section_nonexistent_section() {
        let mut library = create_library();
        let game = test_game("1", "Monkey Island", "monkey-island");

        library.add_game(game);

        let cursor = library.to_section("Z");

        assert!(cursor.is_none());
    }

    #[test]
    fn test_to_section_empty_library() {
        let library = create_library();

        let cursor = library.to_section("M");

        assert!(cursor.is_none());
    }

    #[test]
    fn test_to_game_finds_existing_game() {
        let mut library = create_library();
        let game1 = test_game("1", "Alice", "alice");
        let game2 = test_game("2", "Monkey Island", "monkey-island");

        library.add_game(game1.clone());
        library.add_game(game2.clone());

        let cursor = library.to_game(game2.id()).unwrap();

        assert_eq!(cursor.section_id(), library.sections[1].id());
        assert_eq!(cursor.game_id(), game2.id());
    }

    #[test]
    fn test_to_game_nonexistent_game() {
        let mut library = create_library();
        let game = test_game("1", "Monkey Island", "monkey-island");

        library.add_game(game);

        let nonexistent_id = GameId::new("999".to_string());
        let cursor = library.to_game(&nonexistent_id);

        assert!(cursor.is_none());
    }

    #[test]
    fn test_to_game_empty_library() {
        let library = create_library();
        let game_id = GameId::new("1".to_string());

        let cursor = library.to_game(&game_id);

        assert!(cursor.is_none());
    }

    #[test]
    fn test_get_cursor_empty_library() {
        let library = create_library();

        let cursor = library.get_cursor();

        assert!(cursor.is_none());
    }

    #[test]
    fn test_get_cursor_single_game() {
        let mut library = create_library();
        let game = test_game("1", "Monkey Island", "monkey-island");

        library.add_game(game.clone());

        let cursor = library.get_cursor().unwrap();

        assert_eq!(cursor.section_id(), library.sections[0].id());
        assert_eq!(cursor.game_id(), game.id());
    }

    #[test]
    fn test_get_cursor_multiple_games_single_section() {
        let mut library = create_library();
        let game1 = test_game("1", "Maniac Mansion", "maniac-mansion");
        let game2 = test_game("2", "Monkey Island", "monkey-island");

        library.add_game(game1.clone());
        library.add_game(game2);

        let cursor = library.get_cursor().unwrap();

        assert_eq!(cursor.section_id(), library.sections[0].id());
        assert_eq!(cursor.game_id(), game1.id());
    }

    #[test]
    fn test_get_cursor_multiple_sections() {
        let mut library = create_library();
        let game1 = test_game("1", "Monkey Island", "monkey-island");
        let game2 = test_game("2", "Alice in Wonderland", "alice-in-wonderland");
        let game3 = test_game("3", "Another World", "another-world");

        library.add_game(game1);
        library.add_game(game2.clone());
        library.add_game(game3);

        let cursor = library.get_cursor().unwrap();

        assert_eq!(cursor.section_id(), library.sections[0].id());
        assert_eq!(cursor.game_id(), game2.id());
    }
}
