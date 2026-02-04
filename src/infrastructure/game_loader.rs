use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::domain::game::{Game, GameId};
use crate::domain::media::{Media, MediaSet, MediaType};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GameConfig {
    id: String,
    title: String,
    sort_title: String,
    year: Option<String>,
    publisher: Option<String>,
    notes: Option<String>,
}

pub fn load_games_from_directory(games_dir: &Path) -> Result<Vec<Game>, String> {
    if !games_dir.exists() {
        return Err(format!("Games directory does not exist: {}", games_dir.display()));
    }

    let mut games = Vec::new();

    let entries = fs::read_dir(games_dir).map_err(|e| format!("Failed to read games directory: {e}"))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {e}"))?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let config_path = path.join("config.json");
        if !config_path.exists() {
            continue;
        }

        match load_game_from_config(&config_path, &path) {
            Ok(game) => games.push(game),
            Err(e) => eprintln!("Failed to load game from {}: {}", config_path.display(), e),
        }
    }

    Ok(games)
}

fn load_game_from_config(config_path: &Path, game_dir: &Path) -> Result<Game, String> {
    let contents = fs::read_to_string(config_path).map_err(|e| format!("Failed to read config file: {e}"))?;

    let config: GameConfig = serde_json::from_str(&contents).map_err(|e| format!("Failed to parse JSON: {e}"))?;

    let year = config.year.and_then(|y| y.parse::<u16>().ok());

    let media_set = load_media_set(game_dir);

    Ok(Game::new(
        GameId::new(config.id),
        config.title,
        config.sort_title,
        year,
        config.publisher,
        config.notes,
        media_set,
        Vec::new(),
    ))
}

fn load_media_set(game_dir: &Path) -> MediaSet {
    let media_dir = game_dir.join("media");

    let box_front_2d = load_media(&media_dir, "2d-box-front", MediaType::BoxFront2D);
    let box_front_2d_thumbnail = load_media(&media_dir, "2d-box-front-thumbnail", MediaType::BoxFront2DThumbnail);
    let screenshot_loading = load_media(&media_dir, "screenshot-loading", MediaType::ScreenshotLoading);
    let screenshot_title = load_media(&media_dir, "screenshot-title", MediaType::ScreenshotTitle);
    let screenshot_gameplay = load_media(&media_dir, "screenshot-gameplay", MediaType::ScreenshotGameplay);

    MediaSet::new(box_front_2d, box_front_2d_thumbnail, screenshot_loading, screenshot_title, screenshot_gameplay)
}

fn load_media(media_dir: &Path, base_name: &str, media_type: MediaType) -> Option<Media> {
    let extensions = ["png", "jpg", "jpeg"];

    for ext in &extensions {
        let path = media_dir.join(format!("{base_name}.{ext}"));
        if path.exists() {
            return Some(Media::new(media_type, path));
        }
    }

    None
}
