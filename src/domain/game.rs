use std::cmp::Ordering;

#[derive(Clone, PartialEq, Eq)]
pub struct Game {
    title: String,
    sort_key: String,
    year: Option<u16>,
    publisher: Option<String>,
    notes: Option<String>,
}

impl Game {
    pub const fn new(
        title: String,
        sort_key: String,
        year: Option<u16>,
        publisher: Option<String>,
        notes: Option<String>,
    ) -> Self {
        Self {
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
        visitor(
            &self.title,
            self.year,
            self.publisher.as_deref(),
            self.notes.as_deref(),
        )
    }

    pub fn starts_with(&self, c: char) -> bool {
        self.sort_key.starts_with(c)
    }

    pub fn same_section(&self, other: &Self) -> bool {
        let self_char = self.sort_key.chars().next();
        let other_char = other.sort_key.chars().next();
        self_char == other_char
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
mod tests {
    use super::*;

    #[test]
    fn test_games_sorted_by_sort_key() {
        let game1 = Game::new(
            "Zak McKracken".to_string(),
            "zak-mckracken".to_string(),
            Some(1988),
            None,
            None,
        );
        let game2 = Game::new(
            "Monkey Island".to_string(),
            "monkey-island".to_string(),
            Some(1990),
            None,
            None,
        );
        let game3 = Game::new(
            "Maniac Mansion".to_string(),
            "maniac-mansion".to_string(),
            Some(1987),
            None,
            None,
        );

        let mut games = vec![game1, game2, game3];
        games.sort();

        assert_eq!(games[0].sort_key, "maniac-mansion");
        assert_eq!(games[1].sort_key, "monkey-island");
        assert_eq!(games[2].sort_key, "zak-mckracken");
    }

    #[test]
    fn test_visitor_with_all_fields() {
        let game = Game::new(
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
        let game = Game::new(
            "Unknown Game".to_string(),
            "unknown-game".to_string(),
            None,
            None,
            None,
        );

        game.visit(|title, year, publisher, notes| {
            assert_eq!(title, "Unknown Game");
            assert_eq!(year, None);
            assert_eq!(publisher, None);
            assert_eq!(notes, None);
        });
    }
}
