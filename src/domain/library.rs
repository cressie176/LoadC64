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

    pub fn add_game(&mut self, game: &Game) {
        for section in &mut self.sections {
            if section.add(game) {
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

    pub fn jump(&mut self, game_id: &str) {
        for (section_idx, section) in self.sections.iter_mut().enumerate() {
            if section.jump(game_id) {
                self.current_section_idx = section_idx;
                return;
            }
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

    pub fn with_games<F>(&mut self, offset: isize, size: usize, mut visitor: F)
    where
        F: FnMut(&Game),
    {
        if self.sections.is_empty() {
            return;
        }

        for _ in 0..offset.unsigned_abs() {
            self.previous_game();
        }

        for _ in 0..size {
            self.with_current_game(|game| visitor(game));
            self.next_game();
        }
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
    use super::super::test_utils::test_game;
    use super::*;

    #[test]
    fn test_next_section_moves_forward() {
        let mut library: Library<CharacterSection> = Library::new();
        let mut section_a = CharacterSection::new('a');
        let mut section_b = CharacterSection::new('b');

        section_a.add(&test_game("a1", "Apple", "apple"));
        section_b.add(&test_game("b1", "Banana", "banana"));

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

        section_a.add(&test_game("a1", "Apple", "apple"));
        section_b.add(&test_game("b1", "Banana", "banana"));

        library.add_section(section_a);
        library.add_section(section_b);

        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Apple".to_string()));

        library.next_section();
        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Banana".to_string()));

        library.next_section();
        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Apple".to_string()));
    }

    #[test]
    fn test_next_section_empty_library() {
        let mut library: Library<CharacterSection> = Library::new();

        library.next_section();
        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, None);
    }

    #[test]
    fn test_previous_section_moves_backward() {
        let mut library: Library<CharacterSection> = Library::new();
        let mut section_a = CharacterSection::new('a');
        let mut section_b = CharacterSection::new('b');

        section_a.add(&test_game("a1", "Apple", "apple"));
        section_b.add(&test_game("b1", "Banana", "banana"));

        library.add_section(section_a);
        library.add_section(section_b);

        library.next_section();
        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Banana".to_string()));

        library.previous_section();
        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Apple".to_string()));
    }

    #[test]
    fn test_previous_section_wraps_to_end() {
        let mut library: Library<CharacterSection> = Library::new();
        let mut section_a = CharacterSection::new('a');
        let mut section_b = CharacterSection::new('b');

        section_a.add(&test_game("a1", "Apple", "apple"));
        section_b.add(&test_game("b1", "Banana", "banana"));

        library.add_section(section_a);
        library.add_section(section_b);

        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Apple".to_string()));

        library.previous_section();
        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Banana".to_string()));

        library.previous_section();
        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Apple".to_string()));
    }

    #[test]
    fn test_previous_section_empty_library() {
        let mut library: Library<CharacterSection> = Library::new();

        library.previous_section();
        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, None);
    }

    #[test]
    fn test_next_game_empty_library() {
        let mut library: Library<CharacterSection> = Library::new();
        library.next_game();
        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, None);
    }

    #[test]
    fn test_next_game_moves_within_section() {
        let mut library: Library<CharacterSection> = Library::new();
        let mut section_a = CharacterSection::new('a');

        section_a.add(&test_game("a1", "Apple", "apple"));
        section_a.add(&test_game("a2", "Aardvark", "aardvark"));
        section_a.add(&test_game("a3", "Ant", "ant"));

        library.add_section(section_a);

        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Aardvark".to_string()));

        library.next_game();
        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Ant".to_string()));

        library.next_game();
        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Apple".to_string()));
    }

    #[test]
    fn test_next_game_moves_to_next_section() {
        let mut library: Library<CharacterSection> = Library::new();
        let mut section_a = CharacterSection::new('a');
        let mut section_b = CharacterSection::new('b');

        section_a.add(&test_game("a1", "Apple", "apple"));
        section_a.add(&test_game("a3", "Ant", "ant"));
        section_b.add(&test_game("b1", "Banana", "banana"));

        library.add_section(section_a);
        library.add_section(section_b);

        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Ant".to_string()));

        library.next_game();
        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Apple".to_string()));

        library.next_game();
        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Banana".to_string()));
    }

    #[test]
    fn test_next_game_wraps_from_last_section_to_first() {
        let mut library: Library<CharacterSection> = Library::new();
        let mut section_a = CharacterSection::new('a');
        let mut section_b = CharacterSection::new('b');

        section_a.add(&test_game("a1", "Apple", "apple"));
        section_b.add(&test_game("b1", "Banana", "banana"));

        library.add_section(section_a);
        library.add_section(section_b);

        library.next_section();
        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Banana".to_string()));

        library.next_game();
        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Apple".to_string()));
    }

    #[test]
    fn test_previous_game_empty_library() {
        let mut library: Library<CharacterSection> = Library::new();
        library.previous_game();
        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, None);
    }

    #[test]
    fn test_previous_game_moves_within_section() {
        let mut library: Library<CharacterSection> = Library::new();
        let mut section_a = CharacterSection::new('a');

        section_a.add(&test_game("a1", "Apple", "apple"));
        section_a.add(&test_game("a2", "Aardvark", "aardvark"));
        section_a.add(&test_game("a3", "Ant", "ant"));

        library.add_section(section_a);

        library.next_game();
        library.next_game();
        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Apple".to_string()));

        library.previous_game();
        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Ant".to_string()));

        library.previous_game();
        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Aardvark".to_string()));
    }

    #[test]
    fn test_previous_game_moves_to_previous_section() {
        let mut library: Library<CharacterSection> = Library::new();
        let mut section_a = CharacterSection::new('a');
        let mut section_b = CharacterSection::new('b');

        section_a.add(&test_game("a1", "Apple", "apple"));
        section_a.add(&test_game("a3", "Ant", "ant"));
        section_b.add(&test_game("b1", "Banana", "banana"));

        library.add_section(section_a);
        library.add_section(section_b);

        library.next_section();
        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Banana".to_string()));

        library.previous_game();
        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Apple".to_string()));
    }

    #[test]
    fn test_previous_game_wraps_from_first_section_to_last() {
        let mut library: Library<CharacterSection> = Library::new();
        let mut section_a = CharacterSection::new('a');
        let mut section_b = CharacterSection::new('b');

        section_a.add(&test_game("a1", "Apple", "apple"));
        section_b.add(&test_game("b1", "Banana", "banana"));

        library.add_section(section_a);
        library.add_section(section_b);

        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Apple".to_string()));

        library.previous_game();
        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Banana".to_string()));
    }

    #[test]
    fn test_jump_to_game_in_same_section() {
        let mut library: Library<CharacterSection> = Library::new();
        let mut section_a = CharacterSection::new('a');

        section_a.add(&test_game("a1", "Apple", "apple"));
        section_a.add(&test_game("a2", "Ant", "ant"));
        section_a.add(&test_game("a3", "Aardvark", "aardvark"));

        library.add_section(section_a);

        library.jump("a1");
        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Apple".to_string()));
    }

    #[test]
    fn test_jump_to_game_in_different_section() {
        let mut library: Library<CharacterSection> = Library::new();
        let mut section_a = CharacterSection::new('a');
        let mut section_b = CharacterSection::new('b');

        section_a.add(&test_game("a1", "Apple", "apple"));
        section_b.add(&test_game("b1", "Banana", "banana"));

        library.add_section(section_a);
        library.add_section(section_b);

        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Apple".to_string()));

        library.jump("b1");
        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Banana".to_string()));
    }

    #[test]
    fn test_jump_to_nonexistent_game() {
        let mut library: Library<CharacterSection> = Library::new();
        let mut section_a = CharacterSection::new('a');

        section_a.add(&test_game("a1", "Apple", "apple"));

        library.add_section(section_a);

        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Apple".to_string()));

        library.jump("nonexistent");
        let title = library.with_current_game(|game| game.visit(|title, _, _, _| title.to_string()));
        assert_eq!(title, Some("Apple".to_string()));
    }

    #[test]
    fn test_with_games_window() {
        let mut library: Library<CharacterSection> = Library::new();
        library.add_section(CharacterSection::new('a'));
        library.add_section(CharacterSection::new('b'));

        library.add_game(&test_game("a1", "Apple", "apple"));
        library.add_game(&test_game("a2", "Ant", "ant"));
        library.add_game(&test_game("a3", "Aardvark", "aardvark"));
        library.add_game(&test_game("b1", "Banana", "banana"));
        library.add_game(&test_game("b2", "Berry", "berry"));

        library.next_game();
        library.next_game();

        let mut titles = Vec::new();
        library.with_games(-1, 3, |game| {
            game.visit(|title, _, _, _| titles.push(title.to_string()));
        });

        assert_eq!(titles.len(), 3);
        assert_eq!(titles[0], "Ant");
        assert_eq!(titles[1], "Apple");
        assert_eq!(titles[2], "Banana");
    }

    #[test]
    fn test_with_games_empty_library() {
        let mut library: Library<CharacterSection> = Library::new();

        let mut count = 0;
        library.with_games(-2, 5, |_game| {
            count += 1;
        });

        assert_eq!(count, 0);
    }

    #[test]
    fn test_with_games_fewer_than_size() {
        let mut library: Library<CharacterSection> = Library::new();
        library.add_section(CharacterSection::new('a'));

        library.add_game(&test_game("a1", "Apple", "apple"));
        library.add_game(&test_game("a2", "Ant", "ant"));

        let mut titles = Vec::new();
        library.with_games(-1, 5, |game| {
            game.visit(|title, _, _, _| titles.push(title.to_string()));
        });

        assert_eq!(titles.len(), 5);
        assert_eq!(titles[0], "Ant");
        assert_eq!(titles[1], "Apple");
        assert_eq!(titles[2], "Ant");
        assert_eq!(titles[3], "Apple");
        assert_eq!(titles[4], "Ant");
    }
}
