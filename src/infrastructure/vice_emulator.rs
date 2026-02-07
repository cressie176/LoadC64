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
        Command::new(&self.executable_path)
            .args([
                "-trapdevice8",
                "-autostart-warp",
                "-VICIIfull",
                "-VICIIfilter",
                "0",
                "-VICIIglfilter",
                "0",
                "-VICIIdscan",
                "-joydev1",
                "0",
                "-joydev2",
                "1",
                "+confirmonexit",
                "-autostart",
                &rom_path.to_string_lossy(),
            ])
            .spawn()
            .map_err(|e| format!("Failed to launch VICE: {e}"))?;

        Ok(())
    }
}
