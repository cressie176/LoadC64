use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::domain::game::{Game, GameId};
use crate::domain::library::Library;
use crate::domain::media::{Media, MediaSet, MediaType};
use crate::domain::rom::Rom;
use crate::domain::section::Section;

#[derive(Debug, Deserialize, Serialize)]
struct GameConfig {
    id: String,
    title: String,
    sort_title: String,
    year: Option<String>,
    publisher: Option<String>,
    notes: Option<String>,
    media: Option<Vec<MediaConfig>>,
    #[serde(default)]
    hidden: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct MediaConfig {
    r#type: String,
    file: String,
}

#[allow(dead_code)]
pub fn load_games_into<S: Section + Ord>(library: &mut Library<S>, games_dir: &Path) -> Result<(), String> {
    if !games_dir.exists() {
        return Err(format!("Games directory does not exist: {}", games_dir.display()));
    }

    let entries = fs::read_dir(games_dir).map_err(|e| format!("Failed to read games directory: {e}"))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {e}"))?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let config_path = path.join("config.toml");
        if !config_path.exists() {
            continue;
        }

        match load_game_from_config(&config_path, &path) {
            Ok(game) => library.add_game(game),
            Err(e) => eprintln!("Failed to load game from {}: {}", config_path.display(), e),
        }
    }

    Ok(())
}

fn load_game_from_config(config_path: &Path, game_dir: &Path) -> Result<Game, String> {
    let contents = fs::read_to_string(config_path).map_err(|e| format!("Failed to read config file: {e}"))?;

    let config: GameConfig = toml::from_str(&contents).map_err(|e| format!("Failed to parse TOML: {e}"))?;

    let year = config.year.and_then(|y| y.parse::<u16>().ok());
    let hidden = config.hidden.unwrap_or(false);

    let media_set = load_media_set(game_dir, config.media.clone());
    let roms = load_roms(game_dir);

    Ok(Game::new(GameId::new(config.id), config.title, config.sort_title, year, config.publisher, config.notes, media_set, roms, game_dir.to_path_buf(), hidden))
}

fn get_default_box_art_path() -> PathBuf {
    let exe_dir = std::env::current_exe().expect("Failed to get executable path").parent().expect("Failed to get executable directory").to_path_buf();

    let release_path = exe_dir.join("assets").join("media").join("default-2d-box-front-thumbnail.png");
    if release_path.exists() {
        return release_path;
    }

    let project_root = exe_dir.parent().and_then(|p| p.parent()).map_or_else(|| exe_dir.clone(), std::path::Path::to_path_buf);
    let dev_path = project_root.join("assets").join("media").join("default-2d-box-front-thumbnail.png");
    if dev_path.exists() {
        return dev_path;
    }

    release_path
}

fn load_media_set(game_dir: &Path, media_configs: Option<Vec<MediaConfig>>) -> MediaSet {
    let media_dir = game_dir.join("media");

    let mut box_front_2d = None;
    let mut box_front_2d_thumbnail = None;
    let mut screenshot_loading = None;
    let mut screenshot_title = None;
    let mut screenshot_gameplay = None;

    let default_files = [
        ("2d-box-front.png", MediaType::BoxFront2D),
        ("2d-box-front.jpg", MediaType::BoxFront2D),
        ("2d-box-front-thumbnail.png", MediaType::BoxFront2DThumbnail),
        ("2d-box-front-thumbnail.jpg", MediaType::BoxFront2DThumbnail),
        ("screenshot-loading.png", MediaType::ScreenshotLoading),
        ("screenshot-title.png", MediaType::ScreenshotTitle),
        ("screenshot-gameplay.png", MediaType::ScreenshotGameplay),
    ];

    for (filename, media_type) in &default_files {
        let media_path = media_dir.join(filename);
        if media_path.exists() {
            match media_type {
                MediaType::BoxFront2D => box_front_2d = Some(Media::new(media_type.clone(), media_path)),
                MediaType::BoxFront2DThumbnail => box_front_2d_thumbnail = Some(Media::new(media_type.clone(), media_path)),
                MediaType::ScreenshotLoading => screenshot_loading = Some(Media::new(media_type.clone(), media_path)),
                MediaType::ScreenshotTitle => screenshot_title = Some(Media::new(media_type.clone(), media_path)),
                MediaType::ScreenshotGameplay => screenshot_gameplay = Some(Media::new(media_type.clone(), media_path)),
            }
        }
    }

    if let Some(configs) = media_configs {
        for config in configs {
            let media_path = media_dir.join(&config.file);
            if !media_path.exists() {
                continue;
            }

            match config.r#type.as_str() {
                "2d-box-front" => box_front_2d = Some(Media::new(MediaType::BoxFront2D, media_path)),
                "2d-box-front-thumbnail" => box_front_2d_thumbnail = Some(Media::new(MediaType::BoxFront2DThumbnail, media_path)),
                "screenshot-loading" => screenshot_loading = Some(Media::new(MediaType::ScreenshotLoading, media_path)),
                "screenshot-title" => screenshot_title = Some(Media::new(MediaType::ScreenshotTitle, media_path)),
                "screenshot-gameplay" => screenshot_gameplay = Some(Media::new(MediaType::ScreenshotGameplay, media_path)),
                _ => eprintln!("Unknown media type: {}", config.r#type),
            }
        }
    }

    let box_front_2d_thumbnail = box_front_2d_thumbnail.unwrap_or_else(|| {
        let default_path = get_default_box_art_path();
        Media::new(MediaType::BoxFront2DThumbnail, default_path)
    });

    MediaSet::new(box_front_2d, box_front_2d_thumbnail, screenshot_loading, screenshot_title, screenshot_gameplay)
}

fn load_roms(game_dir: &Path) -> Vec<Rom> {
    let roms_dir = game_dir.join("roms");
    let mut roms = Vec::new();

    if let Ok(entries) = fs::read_dir(&roms_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file()
                && let Some(ext) = path.extension()
            {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if ext_str == "d64" || ext_str == "t64" || ext_str == "prg" || ext_str == "crt" {
                    roms.push(Rom::new(path));
                }
            }
        }
    }

    roms
}

pub fn load_all_games(games_dir: &Path) -> Result<Vec<Game>, String> {
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

        let config_path = path.join("config.toml");
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

pub fn save_game_config(game: &Game) -> Result<(), String> {
    let config_path = game.game_dir().join("config.toml");

    let contents = fs::read_to_string(&config_path).map_err(|e| format!("Failed to read config file: {e}"))?;

    let mut config: GameConfig = toml::from_str(&contents).map_err(|e| format!("Failed to parse TOML: {e}"))?;

    config.hidden = Some(game.is_hidden());

    let toml_string = toml::to_string_pretty(&config).map_err(|e| format!("Failed to serialize config: {e}"))?;

    fs::write(&config_path, toml_string).map_err(|e| format!("Failed to write config: {e}"))?;

    Ok(())
}
