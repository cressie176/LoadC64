use super::game::Game;

pub trait Section {
    fn add(&mut self, game: &Game) -> bool;
    fn next(&mut self) -> bool;
    fn previous(&mut self) -> bool;
    fn first(&mut self);
    fn last(&mut self);
    fn with_current_game<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&Game) -> R;
}
