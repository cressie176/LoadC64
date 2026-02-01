use super::game::Game;
use super::section::Section;

pub struct Library<S: Section + Ord> {
    sections: Vec<S>,
    current_section_idx: usize,
}

impl<S: Section + Ord> Library<S> {
    pub const fn new() -> Self {
        Self {
            sections: Vec::new(),
            current_section_idx: 0,
        }
    }

    pub fn add_section(&mut self, section: S) {
        self.sections.push(section);
        self.sections.sort();
    }

    pub fn next_section(&mut self) {
        if !self.sections.is_empty() {
            self.increment_current_section_idx();
            self.current_section_mut().first();
        }
    }

    pub fn previous_section(&mut self) {
        if !self.sections.is_empty() {
            self.decrement_current_section_idx();
            self.current_section_mut().first();
        }
    }

    pub fn add_game(&mut self, game: Game) {
        for section in &mut self.sections {
            if section.add(&game) {
                break;
            }
        }
    }

    pub fn next_game(&mut self) {
        if self.sections.is_empty() {
            return;
        }
        let moved = self.current_section_mut().next();
        if !moved {
            self.increment_current_section_idx();
            self.current_section_mut().first();
        }
    }

    pub fn previous_game(&mut self) {
        if self.sections.is_empty() {
            return;
        }
        let moved = self.current_section_mut().previous();
        if !moved {
            self.decrement_current_section_idx();
            self.current_section_mut().last();
        }
    }

    pub fn with_current_game<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&Game) -> R,
    {
        if self.sections.is_empty() {
            return None;
        }
        self.current_section().with_current_game(f)
    }

    const fn increment_current_section_idx(&mut self) {
        self.current_section_idx = (self.current_section_idx + 1) % self.sections.len();
    }

    const fn decrement_current_section_idx(&mut self) {
        self.current_section_idx =
            (self.current_section_idx + self.sections.len() - 1) % self.sections.len();
    }

    fn current_section(&self) -> &S {
        &self.sections[self.current_section_idx]
    }

    fn current_section_mut(&mut self) -> &mut S {
        &mut self.sections[self.current_section_idx]
    }
}

#[cfg(test)]
mod tests {
    use super::super::character_section::CharacterSection;
    use super::*;

    #[test]
    fn test_next_section_moves_forward() {
        let mut library: Library<CharacterSection> = Library::new();
        let mut section_a = CharacterSection::new('a');
        let mut section_b = CharacterSection::new('b');

        section_a.add(&Game::new(
            "Apple".to_string(),
            "apple".to_string(),
            None,
            None,
            None,
        ));
        section_b.add(&Game::new(
            "Banana".to_string(),
            "banana".to_string(),
            None,
            None,
            None,
        ));

        library.add_section(section_a);
        library.add_section(section_b);

        let title =
            library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Apple".to_string()));

        library.next_section();
        let title =
            library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Banana".to_string()));
    }

    #[test]
    fn test_next_section_wraps_to_start() {
        let mut library: Library<CharacterSection> = Library::new();
        let mut section_a = CharacterSection::new('a');
        let mut section_b = CharacterSection::new('b');

        section_a.add(&Game::new(
            "Apple".to_string(),
            "apple".to_string(),
            None,
            None,
            None,
        ));
        section_b.add(&Game::new(
            "Banana".to_string(),
            "banana".to_string(),
            None,
            None,
            None,
        ));

        library.add_section(section_a);
        library.add_section(section_b);

        let title =
            library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Apple".to_string()));

        library.next_section();
        let title =
            library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Banana".to_string()));

        library.next_section();
        let title =
            library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Apple".to_string()));
    }

    #[test]
    fn test_next_section_empty_library() {
        let mut library: Library<CharacterSection> = Library::new();

        library.next_section();
        let title =
            library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, None);
    }

    #[test]
    fn test_previous_section_moves_backward() {
        let mut library: Library<CharacterSection> = Library::new();
        let mut section_a = CharacterSection::new('a');
        let mut section_b = CharacterSection::new('b');

        section_a.add(&Game::new(
            "Apple".to_string(),
            "apple".to_string(),
            None,
            None,
            None,
        ));
        section_b.add(&Game::new(
            "Banana".to_string(),
            "banana".to_string(),
            None,
            None,
            None,
        ));

        library.add_section(section_a);
        library.add_section(section_b);

        library.next_section();
        let title =
            library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Banana".to_string()));

        library.previous_section();
        let title =
            library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Apple".to_string()));
    }

    #[test]
    fn test_previous_section_wraps_to_end() {
        let mut library: Library<CharacterSection> = Library::new();
        let mut section_a = CharacterSection::new('a');
        let mut section_b = CharacterSection::new('b');

        section_a.add(&Game::new(
            "Apple".to_string(),
            "apple".to_string(),
            None,
            None,
            None,
        ));
        section_b.add(&Game::new(
            "Banana".to_string(),
            "banana".to_string(),
            None,
            None,
            None,
        ));

        library.add_section(section_a);
        library.add_section(section_b);

        let title =
            library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Apple".to_string()));

        library.previous_section();
        let title =
            library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Banana".to_string()));

        library.previous_section();
        let title =
            library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Apple".to_string()));
    }

    #[test]
    fn test_previous_section_empty_library() {
        let mut library: Library<CharacterSection> = Library::new();

        library.previous_section();
        let title =
            library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, None);
    }

    #[test]
    fn test_next_game_empty_library() {
        let mut library: Library<CharacterSection> = Library::new();
        library.next_game();
        let title =
            library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, None);
    }

    #[test]
    fn test_next_game_moves_within_section() {
        let mut library: Library<CharacterSection> = Library::new();
        let mut section_a = CharacterSection::new('a');

        section_a.add(&Game::new(
            "Apple".to_string(),
            "apple".to_string(),
            None,
            None,
            None,
        ));
        section_a.add(&Game::new(
            "Aardvark".to_string(),
            "aardvark".to_string(),
            None,
            None,
            None,
        ));
        section_a.add(&Game::new(
            "Ant".to_string(),
            "ant".to_string(),
            None,
            None,
            None,
        ));

        library.add_section(section_a);

        let title =
            library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Aardvark".to_string()));

        library.next_game();
        let title =
            library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Ant".to_string()));

        library.next_game();
        let title =
            library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Apple".to_string()));
    }

    #[test]
    fn test_next_game_moves_to_next_section() {
        let mut library: Library<CharacterSection> = Library::new();
        let mut section_a = CharacterSection::new('a');
        let mut section_b = CharacterSection::new('b');

        section_a.add(&Game::new(
            "Apple".to_string(),
            "apple".to_string(),
            None,
            None,
            None,
        ));
        section_a.add(&Game::new(
            "Ant".to_string(),
            "ant".to_string(),
            None,
            None,
            None,
        ));
        section_b.add(&Game::new(
            "Banana".to_string(),
            "banana".to_string(),
            None,
            None,
            None,
        ));

        library.add_section(section_a);
        library.add_section(section_b);

        let title =
            library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Ant".to_string()));

        library.next_game();
        let title =
            library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Apple".to_string()));

        library.next_game();
        let title =
            library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Banana".to_string()));
    }

    #[test]
    fn test_next_game_wraps_from_last_section_to_first() {
        let mut library: Library<CharacterSection> = Library::new();
        let mut section_a = CharacterSection::new('a');
        let mut section_b = CharacterSection::new('b');

        section_a.add(&Game::new(
            "Apple".to_string(),
            "apple".to_string(),
            None,
            None,
            None,
        ));
        section_b.add(&Game::new(
            "Banana".to_string(),
            "banana".to_string(),
            None,
            None,
            None,
        ));

        library.add_section(section_a);
        library.add_section(section_b);

        library.next_section();
        let title =
            library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Banana".to_string()));

        library.next_game();
        let title =
            library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Apple".to_string()));
    }

    #[test]
    fn test_previous_game_empty_library() {
        let mut library: Library<CharacterSection> = Library::new();
        library.previous_game();
        let title =
            library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, None);
    }

    #[test]
    fn test_previous_game_moves_within_section() {
        let mut library: Library<CharacterSection> = Library::new();
        let mut section_a = CharacterSection::new('a');

        section_a.add(&Game::new(
            "Apple".to_string(),
            "apple".to_string(),
            None,
            None,
            None,
        ));
        section_a.add(&Game::new(
            "Aardvark".to_string(),
            "aardvark".to_string(),
            None,
            None,
            None,
        ));
        section_a.add(&Game::new(
            "Ant".to_string(),
            "ant".to_string(),
            None,
            None,
            None,
        ));

        library.add_section(section_a);

        library.next_game();
        library.next_game();
        let title =
            library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Apple".to_string()));

        library.previous_game();
        let title =
            library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Ant".to_string()));

        library.previous_game();
        let title =
            library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Aardvark".to_string()));
    }

    #[test]
    fn test_previous_game_moves_to_previous_section() {
        let mut library: Library<CharacterSection> = Library::new();
        let mut section_a = CharacterSection::new('a');
        let mut section_b = CharacterSection::new('b');

        section_a.add(&Game::new(
            "Apple".to_string(),
            "apple".to_string(),
            None,
            None,
            None,
        ));
        section_a.add(&Game::new(
            "Ant".to_string(),
            "ant".to_string(),
            None,
            None,
            None,
        ));
        section_b.add(&Game::new(
            "Banana".to_string(),
            "banana".to_string(),
            None,
            None,
            None,
        ));

        library.add_section(section_a);
        library.add_section(section_b);

        library.next_section();
        let title =
            library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Banana".to_string()));

        library.previous_game();
        let title =
            library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Apple".to_string()));
    }

    #[test]
    fn test_previous_game_wraps_from_first_section_to_last() {
        let mut library: Library<CharacterSection> = Library::new();
        let mut section_a = CharacterSection::new('a');
        let mut section_b = CharacterSection::new('b');

        section_a.add(&Game::new(
            "Apple".to_string(),
            "apple".to_string(),
            None,
            None,
            None,
        ));
        section_b.add(&Game::new(
            "Banana".to_string(),
            "banana".to_string(),
            None,
            None,
            None,
        ));

        library.add_section(section_a);
        library.add_section(section_b);

        let title =
            library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Apple".to_string()));

        library.previous_game();
        let title =
            library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Banana".to_string()));
    }
}
