#[cfg(test)]
pub fn test_game(id: &str, title: &str, sort_key: &str) -> super::game::Game {
    super::game::Game::new(
        id.to_string(),
        title.to_string(),
        sort_key.to_string(),
        None,
        None,
        None,
    )
}
