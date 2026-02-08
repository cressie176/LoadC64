use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ViceConfigFile {
    vice: ViceSection,
    #[serde(default)]
    inherits: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ViceSection {
    arg: Vec<ViceArg>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ViceArg {
    values: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ViceConfig {
    pub args: Vec<Vec<String>>,
}

impl ViceConfig {
    #[cfg(test)]
    pub const fn new(args: Vec<Vec<String>>) -> Self {
        Self { args }
    }

    fn is_removal(arg: &[String]) -> bool {
        arg.first().is_some_and(|s| s.starts_with('!'))
    }

    fn normalize_key(s: &str) -> &str {
        let without_bang = s.strip_prefix('!').unwrap_or(s);
        without_bang.strip_prefix('-').or_else(|| without_bang.strip_prefix('+')).unwrap_or(without_bang)
    }

    fn key(arg: &[String]) -> Option<&str> {
        arg.first().map(|s| Self::normalize_key(s))
    }

    pub fn merge(&mut self, other: &Self) {
        for other_arg in &other.args {
            let other_key = Self::key(other_arg);

            if Self::is_removal(other_arg) {
                self.args.retain(|arg| Self::key(arg) != other_key);
            } else {
                self.args.retain(|arg| Self::key(arg) != other_key);
                self.args.push(other_arg.clone());
            }
        }
    }

    pub fn to_command_args(&self) -> Vec<String> {
        let mut result = Vec::new();

        let (autostart, regular): (Vec<_>, Vec<_>) = self.args.iter().filter(|arg| !Self::is_removal(arg)).partition(|arg| Self::key(arg) == Some("autostart"));

        for arg in &regular {
            result.extend((*arg).clone());
        }

        for arg in &autostart {
            result.extend((*arg).clone());
        }

        result
    }

    pub fn load_default() -> Result<Self, String> {
        let toml_str = include_str!("../../assets/vice/default.toml");
        let file: ViceConfigFile = toml::from_str(toml_str).map_err(|e| e.to_string())?;
        Ok(Self { args: file.vice.arg.into_iter().map(|a| a.values).collect() })
    }

    pub fn load_with_profiles(games_root: &Path, game_dir: &Path) -> Result<Self, String> {
        let game_config_path = game_dir.join("vice.toml");

        if game_config_path.exists() {
            Self::load_from_file(&game_config_path, Some(games_root)).and_then(|opt| opt.ok_or_else(|| "Failed to load game config".to_string()))
        } else {
            Self::load_default_from_games_root(games_root)
        }
    }

    fn load_default_from_games_root(games_root: &Path) -> Result<Self, String> {
        let default_path = games_root.join("default-config.toml");

        if !default_path.exists() {
            return Self::load_default();
        }

        let toml_str = std::fs::read_to_string(&default_path).map_err(|e| e.to_string())?;
        let file: ViceConfigFile = toml::from_str(&toml_str).map_err(|e| e.to_string())?;
        Ok(Self { args: file.vice.arg.into_iter().map(|a| a.values).collect() })
    }

    fn load_from_file(path: &Path, games_root: Option<&Path>) -> Result<Option<Self>, String> {
        let toml_str = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let file: ViceConfigFile = toml::from_str(&toml_str).map_err(|e| e.to_string())?;

        let should_load_default = file.inherits.as_ref().is_none_or(|inherits| !inherits.is_empty());

        let mut config = if should_load_default {
            if let Some(root) = games_root {
                let default_path = root.join("default-config.toml");
                if default_path.exists() { Self::load_default_from_games_root(root)? } else { Self { args: Vec::new() } }
            } else {
                Self { args: Vec::new() }
            }
        } else {
            Self { args: Vec::new() }
        };

        if let Some(inherits) = &file.inherits
            && let Some(root) = games_root
        {
            let profiles_dir = root.join("profiles");
            for profile_name in inherits {
                let profile_path = profiles_dir.join(format!("{profile_name}.toml"));
                if profile_path.exists()
                    && let Some(profile_config) = Self::load_profile(&profile_path)?
                {
                    config.merge(&profile_config);
                }
            }
        }

        let file_config = Self { args: file.vice.arg.into_iter().map(|a| a.values).collect() };
        config.merge(&file_config);

        Ok(Some(config))
    }

    fn load_profile(path: &Path) -> Result<Option<Self>, String> {
        let toml_str = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let file: ViceConfigFile = toml::from_str(&toml_str).map_err(|e| e.to_string())?;
        Ok(Some(Self { args: file.vice.arg.into_iter().map(|a| a.values).collect() }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn arg(parts: &[&str]) -> Vec<String> {
        parts.iter().map(|s| (*s).to_string()).collect()
    }

    #[test]
    fn test_merge_replaces_existing_arg() {
        let mut config = ViceConfig::new(vec![arg(&["-joydev1", "0"]), arg(&["-VICIIfilter", "0"])]);

        let override_config = ViceConfig::new(vec![arg(&["-joydev1", "1"])]);

        config.merge(&override_config);

        assert_eq!(config.args, vec![arg(&["-VICIIfilter", "0"]), arg(&["-joydev1", "1"])]);
    }

    #[test]
    fn test_merge_removes_arg_with_bang_prefix() {
        let mut config = ViceConfig::new(vec![arg(&["-trapdevice8"]), arg(&["-autostart-warp"]), arg(&["-VICIIfilter", "0"])]);

        let override_config = ViceConfig::new(vec![arg(&["!-autostart-warp"])]);

        config.merge(&override_config);

        assert_eq!(config.args, vec![arg(&["-trapdevice8"]), arg(&["-VICIIfilter", "0"])]);
    }

    #[test]
    fn test_merge_adds_new_arg() {
        let mut config = ViceConfig::new(vec![arg(&["-joydev1", "0"])]);

        let override_config = ViceConfig::new(vec![arg(&["-sound"])]);

        config.merge(&override_config);

        assert_eq!(config.args, vec![arg(&["-joydev1", "0"]), arg(&["-sound"])]);
    }

    #[test]
    fn test_to_command_args_flattens_args() {
        let config = ViceConfig::new(vec![arg(&["-trapdevice8"]), arg(&["-VICIIfilter", "0"]), arg(&["-joydev1", "0"])]);

        let result = config.to_command_args();

        assert_eq!(result, vec!["-trapdevice8", "-VICIIfilter", "0", "-joydev1", "0"]);
    }

    #[test]
    fn test_to_command_args_filters_out_removals() {
        let config = ViceConfig::new(vec![arg(&["-trapdevice8"]), arg(&["!-autostart-warp"]), arg(&["-VICIIfilter", "0"])]);

        let result = config.to_command_args();

        assert_eq!(result, vec!["-trapdevice8", "-VICIIfilter", "0"]);
    }

    #[test]
    fn test_to_command_args_puts_autostart_last() {
        let config = ViceConfig::new(vec![arg(&["-trapdevice8"]), arg(&["-autostart", "game.prg"]), arg(&["-VICIIfilter", "0"])]);

        let result = config.to_command_args();

        assert_eq!(result, vec!["-trapdevice8", "-VICIIfilter", "0", "-autostart", "game.prg"]);
    }

    #[test]
    fn test_complex_merge_scenario() {
        let mut config = ViceConfig::new(vec![arg(&["-trapdevice8"]), arg(&["-autostart-warp"]), arg(&["-VICIIfilter", "0"]), arg(&["-joydev1", "0"])]);

        let override_config = ViceConfig::new(vec![arg(&["-joydev1", "1"]), arg(&["-sound"]), arg(&["!-autostart-warp"])]);

        config.merge(&override_config);

        assert_eq!(config.args, vec![arg(&["-trapdevice8"]), arg(&["-VICIIfilter", "0"]), arg(&["-joydev1", "1"]), arg(&["-sound"])]);
    }

    #[test]
    fn test_remove_matches_plus_and_minus_prefix() {
        let mut config = ViceConfig::new(vec![arg(&["-autostart-warp"]), arg(&["+confirmonexit"]), arg(&["-VICIIfilter", "0"])]);

        let override_config = ViceConfig::new(vec![arg(&["!autostart-warp"]), arg(&["!confirmonexit"])]);

        config.merge(&override_config);

        assert_eq!(config.args, vec![arg(&["-VICIIfilter", "0"])]);
    }

    #[test]
    fn test_remove_with_prefix_also_matches() {
        let mut config = ViceConfig::new(vec![arg(&["-autostart-warp"]), arg(&["+confirmonexit"])]);

        let override_config = ViceConfig::new(vec![arg(&["!-autostart-warp"]), arg(&["!+confirmonexit"])]);

        config.merge(&override_config);

        assert_eq!(config.args, Vec::<Vec<String>>::new());
    }

    #[test]
    fn test_replace_matches_regardless_of_prefix() {
        let mut config = ViceConfig::new(vec![arg(&["+confirmonexit"])]);

        let override_config = ViceConfig::new(vec![arg(&["-confirmonexit"])]);

        config.merge(&override_config);

        assert_eq!(config.args, vec![arg(&["-confirmonexit"])]);
    }

    #[test]
    fn test_load_with_profiles() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let games_root = temp_dir.path();

        let default_config = r#"
[[vice.arg]]
values = ["-trapdevice8"]

[[vice.arg]]
values = ["-autostart-warp"]
"#;
        fs::write(games_root.join("default-config.toml"), default_config).unwrap();

        let profiles_dir = games_root.join("profiles");
        fs::create_dir(&profiles_dir).unwrap();

        let pal_profile = r#"
[[vice.arg]]
values = ["-VICIIfilter", "1"]
"#;
        fs::write(profiles_dir.join("pal.toml"), pal_profile).unwrap();

        let game_dir = games_root.join("game1");
        fs::create_dir(&game_dir).unwrap();

        let game_config = r#"
inherits = ["pal"]

[[vice.arg]]
values = ["-joydev1", "1"]
"#;
        fs::write(game_dir.join("vice.toml"), game_config).unwrap();

        let config = ViceConfig::load_with_profiles(games_root, &game_dir).unwrap();

        assert_eq!(config.args, vec![arg(&["-trapdevice8"]), arg(&["-autostart-warp"]), arg(&["-VICIIfilter", "1"]), arg(&["-joydev1", "1"])]);
    }

    #[test]
    fn test_load_with_profiles_empty_inherits_skips_default() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let games_root = temp_dir.path();

        let default_config = r#"
[[vice.arg]]
values = ["-trapdevice8"]
"#;
        fs::write(games_root.join("default-config.toml"), default_config).unwrap();

        let game_dir = games_root.join("game1");
        fs::create_dir(&game_dir).unwrap();

        let game_config = r#"
inherits = []

[[vice.arg]]
values = ["-joydev1", "1"]
"#;
        fs::write(game_dir.join("vice.toml"), game_config).unwrap();

        let config = ViceConfig::load_with_profiles(games_root, &game_dir).unwrap();

        assert_eq!(config.args, vec![arg(&["-joydev1", "1"])]);
    }

    #[test]
    fn test_load_with_profiles_removal_syntax() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let games_root = temp_dir.path();

        let default_config = r#"
[[vice.arg]]
values = ["-trapdevice8"]

[[vice.arg]]
values = ["-autostart-warp"]
"#;
        fs::write(games_root.join("default-config.toml"), default_config).unwrap();

        let game_dir = games_root.join("game1");
        fs::create_dir(&game_dir).unwrap();

        let game_config = r#"
[[vice.arg]]
values = ["!autostart-warp"]

[[vice.arg]]
values = ["-joydev1", "1"]
"#;
        fs::write(game_dir.join("vice.toml"), game_config).unwrap();

        let config = ViceConfig::load_with_profiles(games_root, &game_dir).unwrap();

        assert_eq!(config.args, vec![arg(&["-trapdevice8"]), arg(&["-joydev1", "1"])]);
    }
}
