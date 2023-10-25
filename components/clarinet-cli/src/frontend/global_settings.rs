use std::fs::{self};

use std::env;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct GlobalSettings {
    pub disable_hints: bool,
    pub enable_telemetry: Option<bool>,
}

impl GlobalSettings {
    pub fn from_global_file() -> Self {
        let home_dir = dirs::home_dir();
        let settings_file = "~/.clarinet/Settings.toml";

        if let Some(path) = home_dir.map(|home_dir| home_dir.join(".clarinet/Settings.toml")) {
            if path.exists() {
                match fs::read_to_string(path) {
                    Ok(content) => match toml::from_str::<GlobalSettings>(&content) {
                        Ok(res) => return res,
                        Err(_) => {
                            println!("{} {}", format_warn!("unable to parse"), settings_file);
                        }
                    },
                    Err(_) => {
                        println!("{} {}", format_warn!("unable to read file"), settings_file);
                    }
                }
            }
        };

        // Keep backwards compatibility with ENV var
        let hints_enabled = env::var("CLARINET_DISABLE_HINTS") != Ok("1".into());
        Self {
            disable_hints: !hints_enabled,
            ..Default::default()
        }
    }
}
