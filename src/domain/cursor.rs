use super::game::GameId;
use super::section::{Section, SectionId};

#[derive(Clone)]
pub struct Cursor {
    section_id: SectionId,
    game_id: GameId,
}

impl Cursor {
    pub const fn new(section_id: SectionId, game_id: GameId) -> Self {
        Self { section_id, game_id }
    }

    pub fn first_game(section: &dyn Section) -> Option<Self> {
        Some(Self { section_id: section.id().clone(), game_id: section.first_game_id()?.clone() })
    }

    pub fn last_game(section: &dyn Section) -> Option<Self> {
        Some(Self { section_id: section.id().clone(), game_id: section.last_game_id()?.clone() })
    }

    pub fn for_game(section: &dyn Section, game_id: &GameId) -> Self {
        Self { section_id: section.id().clone(), game_id: game_id.clone() }
    }

    pub const fn section_id(&self) -> &SectionId {
        &self.section_id
    }

    pub const fn game_id(&self) -> &GameId {
        &self.game_id
    }
}
