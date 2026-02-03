use std::cmp::Ordering;
use std::hash::Hash;

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
}

impl Game {
    pub const fn new(id: GameId, title: String, sort_key: String, year: Option<u16>, publisher: Option<String>, notes: Option<String>) -> Self {
        Self {
            id,
            title,
            sort_key,
            year,
            publisher,
            notes,
        }
    }

    pub fn visit<F, R>(&self, visitor: F) -> R
    where
        F: FnOnce(&str, Option<u16>, Option<&str>, Option<&str>) -> R,
    {
        visitor(&self.title, self.year, self.publisher.as_deref(), self.notes.as_deref())
    }

    pub const fn id(&self) -> &GameId {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn first_character(&self) -> char {
        self.sort_key.chars().next().unwrap_or(' ')
    }

    pub fn starts_with(&self, c: char) -> bool {
        self.sort_key.starts_with(c)
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
    Game::new(GameId::new(id.to_string()), title.to_string(), sort_key.to_string(), None, None, None)
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
    fn test_visitor_with_all_fields() {
        let game = Game::new(
            GameId::new("1".to_string()),
            "Monkey Island".to_string(),
            "monkey-island".to_string(),
            Some(1990),
            Some("LucasArts".to_string()),
            Some("Classic adventure game".to_string()),
        );

        let result = game.visit(|title, year, publisher, notes| {
            assert_eq!(title, "Monkey Island");
            assert_eq!(year, Some(1990));
            assert_eq!(publisher, Some("LucasArts"));
            assert_eq!(notes, Some("Classic adventure game"));
            "visited"
        });

        assert_eq!(result, "visited");
    }

    #[test]
    fn test_visitor_with_none_fields() {
        let game = test_game("1", "Unknown Game", "unknown-game");

        game.visit(|title, year, publisher, notes| {
            assert_eq!(title, "Unknown Game");
            assert_eq!(year, None);
            assert_eq!(publisher, None);
            assert_eq!(notes, None);
        });
    }
}
