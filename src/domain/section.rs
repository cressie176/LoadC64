use std::collections::HashMap;
use uuid::Uuid;

use super::game::{Game, GameId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SectionId(Uuid);

impl SectionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

fn natural_game_order(a: &GameId, b: &GameId, games: &HashMap<GameId, Game>) -> std::cmp::Ordering {
    let game_a = &games[a];
    let game_b = &games[b];
    game_a.cmp(game_b)
}

pub trait Section {
    fn id(&self) -> &SectionId;
    fn title(&self) -> String;
    fn accepts(&self, game: &Game) -> bool;
    fn add_game(&mut self, game: &Game, games: &HashMap<GameId, Game>) -> Result<(), String>;
    fn first_game_id(&self) -> Option<&GameId>;
    fn last_game_id(&self) -> Option<&GameId>;
    fn next_game_id(&self, current_game_id: &GameId) -> Option<&GameId>;
    fn previous_game_id(&self, current_game_id: &GameId) -> Option<&GameId>;
}

pub struct CharacterSection {
    id: SectionId,
    character: char,
    game_ids: Vec<GameId>,
}

impl Ord for CharacterSection {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.character.cmp(&other.character)
    }
}

impl PartialOrd for CharacterSection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CharacterSection {
    fn eq(&self, other: &Self) -> bool {
        self.character == other.character
    }
}

impl Eq for CharacterSection {}

impl CharacterSection {
    pub fn new(game: &Game) -> Self {
        let character = game.first_character().to_uppercase().next().unwrap();
        Self {
            id: SectionId::new(),
            character,
            game_ids: Vec::new(),
        }
    }
}

impl Section for CharacterSection {
    fn id(&self) -> &SectionId {
        &self.id
    }

    fn title(&self) -> String {
        format!("Section '{}'", self.character)
    }

    fn accepts(&self, game: &Game) -> bool {
        let game_char = game.first_character().to_uppercase().next().unwrap();
        self.character == game_char
    }

    fn add_game(&mut self, game: &Game, games: &HashMap<GameId, Game>) -> Result<(), String> {
        if !self.accepts(game) {
            return Err(format!("Game '{}' does not belong in {}", game.title(), self.title()));
        }
        self.game_ids.push(game.id().clone());
        self.game_ids.sort_by(|a, b| natural_game_order(a, b, games));
        Ok(())
    }

    fn first_game_id(&self) -> Option<&GameId> {
        self.game_ids.first()
    }

    fn last_game_id(&self) -> Option<&GameId> {
        self.game_ids.last()
    }

    fn next_game_id(&self, current_game_id: &GameId) -> Option<&GameId> {
        let current_index = self.game_ids.iter().position(|id| id == current_game_id)?;
        let next_index = current_index + 1;
        self.game_ids.get(next_index)
    }

    fn previous_game_id(&self, current_game_id: &GameId) -> Option<&GameId> {
        let current_index = self.game_ids.iter().position(|id| id == current_game_id)?;
        if current_index == 0 {
            return None;
        }
        let prev_index = current_index - 1;
        self.game_ids.get(prev_index)
    }
}

#[cfg(test)]
mod tests {
    use super::super::game::test_game;
    use super::*;

    #[test]
    fn test_character_section_title() {
        let game = test_game("1", "Monkey Island", "monkey-island");
        let section = CharacterSection::new(&game);

        assert_eq!(section.title(), "Section 'M'");
    }

    #[test]
    fn test_accepts_game_with_matching_character() {
        let game1 = test_game("1", "Monkey Island", "monkey-island");
        let section = CharacterSection::new(&game1);

        let game2 = test_game("2", "Maniac Mansion", "maniac-mansion");
        assert!(section.accepts(&game2));
    }

    #[test]
    fn test_rejects_game_with_different_character() {
        let game1 = test_game("1", "Monkey Island", "monkey-island");
        let section = CharacterSection::new(&game1);

        let game2 = test_game("2", "Zak McKracken", "zak-mckracken");
        assert!(!section.accepts(&game2));
    }

    #[test]
    fn test_add_game_to_section() {
        let game = test_game("1", "Monkey Island", "monkey-island");
        let mut games = HashMap::new();
        games.insert(game.id().clone(), game.clone());

        let mut section = CharacterSection::new(&game);
        let result = section.add_game(&game, &games);

        assert!(result.is_ok());
    }

    #[test]
    fn test_add_incompatible_game_returns_error() {
        let game1 = test_game("1", "Monkey Island", "monkey-island");
        let game2 = test_game("2", "Zak McKracken", "zak-mckracken");
        let mut games = HashMap::new();
        games.insert(game2.id().clone(), game2.clone());

        let mut section = CharacterSection::new(&game1);
        let result = section.add_game(&game2, &games);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Game 'Zak McKracken' does not belong in Section 'M'");
    }

    #[test]
    fn test_games_are_sorted_after_adding() {
        let game1 = test_game("1", "Zork", "zork");
        let game2 = test_game("2", "Zelda", "zelda");
        let game3 = test_game("3", "Zoo Keeper", "zoo-keeper");

        let mut games = HashMap::new();
        games.insert(game1.id().clone(), game1.clone());
        games.insert(game2.id().clone(), game2.clone());
        games.insert(game3.id().clone(), game3.clone());

        let mut section = CharacterSection::new(&game1);
        section.add_game(&game3, &games).unwrap();
        section.add_game(&game2, &games).unwrap();
        section.add_game(&game1, &games).unwrap();

        assert_eq!(section.game_ids[0], *game2.id());
        assert_eq!(section.game_ids[1], *game3.id());
        assert_eq!(section.game_ids[2], *game1.id());
    }

    #[test]
    fn test_section_case_insensitive() {
        let game_lower = test_game("1", "monkey island", "monkey-island");
        let game_upper = test_game("2", "Maniac Mansion", "Maniac-Mansion");

        let section_lower = CharacterSection::new(&game_lower);
        let section_upper = CharacterSection::new(&game_upper);

        assert!(section_lower.accepts(&game_upper));
        assert!(section_upper.accepts(&game_lower));
        assert_eq!(section_lower.cmp(&section_upper), std::cmp::Ordering::Equal);
    }
}
