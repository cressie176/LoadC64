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

    pub fn launch(&self, games_root: &Path, rom_path: &Path) -> Result<(), String> {
        let game_dir = rom_path.parent().ok_or_else(|| "Failed to get game directory".to_string())?;
        let config = ViceConfig::load_with_profiles(games_root, game_dir)?;

        self.launch_with_config(rom_path, &config)
    }

    pub fn launch_with_config(&self, rom_path: &Path, config: &ViceConfig) -> Result<(), String> {
        let mut args = config.to_command_args();

        args.push("-remotemonitor".to_string());
        args.push("-remotemonitoraddress".to_string());
        args.push("127.0.0.1:6510".to_string());

        args.push("-autostart".to_string());
        args.push(rom_path.to_string_lossy().to_string());

        Command::new(&self.executable_path).args(args).spawn().map_err(|e| format!("Failed to launch VICE: {e}"))?;

        Ok(())
    }
}
