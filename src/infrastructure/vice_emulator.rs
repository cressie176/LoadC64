use crate::infrastructure::vice_config::ViceConfig;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct ViceEmulator {
    executable_path: PathBuf,
}

impl ViceEmulator {
    pub const fn new(executable_path: PathBuf) -> Self {
        Self { executable_path }
    }

    pub fn launch(&self, rom_path: &Path) -> Result<(), String> {
        let mut config = ViceConfig::load_default()?;

        // Load and merge game-specific override if it exists
        if let Some(game_dir) = rom_path.parent()
            && let Some(game_override) = ViceConfig::load_game_override(game_dir)?
        {
            config.merge(&game_override);
        }

        self.launch_with_config(rom_path, &config)
    }

    pub fn launch_with_config(&self, rom_path: &Path, config: &ViceConfig) -> Result<(), String> {
        let mut args = config.to_command_args();

        args.push("-autostart".to_string());
        args.push(rom_path.to_string_lossy().to_string());

        Command::new(&self.executable_path).args(args).spawn().map_err(|e| format!("Failed to launch VICE: {e}"))?;

        Ok(())
    }
}
