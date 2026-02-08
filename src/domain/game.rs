use std::cmp::Ordering;
use std::hash::Hash;
use std::path::{Path, PathBuf};

use super::media::MediaSet;
use super::rom::Rom;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameId(String);

impl GameId {
    pub const fn new(id: String) -> Self {
        Self(id)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Game {
    id: GameId,
    title: String,
    sort_key: String,
    year: Option<u16>,
    publisher: Option<String>,
    notes: Option<String>,
    media_set: MediaSet,
    roms: Vec<Rom>,
    dir: PathBuf,
    hidden: bool,
}

impl Game {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        id: GameId,
        title: String,
        sort_key: String,
        year: Option<u16>,
        publisher: Option<String>,
        notes: Option<String>,
        media_set: MediaSet,
        roms: Vec<Rom>,
        dir: PathBuf,
        hidden: bool,
    ) -> Self {
        Self { id, title, sort_key, year, publisher, notes, media_set, roms, dir, hidden }
    }

    pub const fn id(&self) -> &GameId {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub const fn year(&self) -> Option<u16> {
        self.year
    }

    pub fn publisher(&self) -> Option<&str> {
        self.publisher.as_deref()
    }

    pub fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }

    pub const fn media_set(&self) -> &MediaSet {
        &self.media_set
    }

    pub fn roms(&self) -> &[Rom] {
        &self.roms
    }

    pub fn first_character(&self) -> char {
        self.sort_key.chars().next().unwrap_or(' ')
    }

    pub fn starts_with(&self, c: char) -> bool {
        self.sort_key.starts_with(c)
    }

    pub fn game_dir(&self) -> &Path {
        &self.dir
    }

    pub const fn is_hidden(&self) -> bool {
        self.hidden
    }

    pub const fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    pub fn sort_key(&self) -> &str {
        &self.sort_key
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sort_key.cmp(&other.sort_key)
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
pub(super) fn test_game(id: &str, title: &str, sort_key: &str) -> Game {
    Game::new(GameId::new(id.to_string()), title.to_string(), sort_key.to_string(), None, None, None, MediaSet::default(), Vec::new(), PathBuf::from("/tmp/test"), false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_games_sorted_by_sort_key() {
        let game1 = test_game("1", "Zak McKracken", "zak-mckracken");
        let game2 = test_game("2", "Monkey Island", "monkey-island");
        let game3 = test_game("3", "Maniac Mansion", "maniac-mansion");

        let mut games = vec![game1, game2, game3];
        games.sort();

        assert!(games[0].starts_with('m'));
        assert!(games[1].starts_with('m'));
        assert!(games[2].starts_with('z'));
    }

    #[test]
    fn test_accessor_with_all_fields() {
        let game = Game::new(
            GameId::new("1".to_string()),
            "Monkey Island".to_string(),
            "monkey-island".to_string(),
            Some(1990),
            Some("LucasArts".to_string()),
            Some("Classic adventure game".to_string()),
            MediaSet::default(),
            Vec::new(),
            PathBuf::from("/tmp/test"),
            false,
        );

        assert_eq!(game.title(), "Monkey Island");
        assert_eq!(game.year(), Some(1990));
        assert_eq!(game.publisher(), Some("LucasArts"));
        assert_eq!(game.notes(), Some("Classic adventure game"));
        assert!(game.media_set().box_front_2d().is_none());
        assert!(game.roms().is_empty());
        assert!(!game.is_hidden());
    }

    #[test]
    fn test_accessor_with_none_fields() {
        let game = test_game("1", "Unknown Game", "unknown-game");

        assert_eq!(game.title(), "Unknown Game");
        assert_eq!(game.year(), None);
        assert_eq!(game.publisher(), None);
        assert_eq!(game.notes(), None);
        assert!(game.media_set().box_front_2d().is_none());
        assert!(game.roms().is_empty());
    }
}
